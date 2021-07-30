use crate::{Finder, Label};
use chrono::{DateTime, Duration, Utc};
use reqwest::{Client, Request, Url};
use std::{
    hash::{Hash, Hasher},
    str::FromStr,
    sync::{Arc, Mutex},
};
use url::Host;

/// Url holder with convenience methods for extractince elements of interest from
/// the url's html
///
/// it's also aware when it was last accessed
#[derive(Debug)]
pub struct Page {
    pub url:  Url,
    pub req:  Option<Request>,
    pub html: Arc<Mutex<Option<String>>>,
    pub last: Arc<Mutex<DateTime<Utc>>>,
}

/// Shorthand for Finder
pub type Find<'a> = &'a Box<dyn Finder>;

impl Page {
    /// Load the page with a Request
    pub fn prep(&mut self, re: Request) -> &mut Self {
        self.req = Some(re);
        self
    }

    /// Loads the html and parsed html in Page preparation for future actions
    pub async fn fetch(&self, client: &Client) -> Self {
        // TODO: Better error recovery with a failures counter in Retriever
        let resp = client
            .execute(self.req.as_ref().unwrap().try_clone().unwrap())
            .await
            .unwrap();
        *self.html.lock().unwrap() = Some(resp.text().await.unwrap());
        *self.last.lock().unwrap() = Utc::now();
        self.to_owned()
    }

    /// Loads the html and parsed html in Page preparation for future actions
    pub async fn fetch_solo(&self) -> Self {
        // TODO: Better error recovery with a failures counter in Retriever
        let resp = Client::new()
            .execute(self.req.as_ref().unwrap().try_clone().unwrap())
            .await
            .unwrap();
        *self.html.lock().unwrap() = Some(resp.text().await.unwrap());
        self.to_owned()
    }

    /// Get the `example.com` from `http://example.com/path/`
    /// would fail for http://localhost/path
    pub fn domain(&self) -> Host {
        let url = self.url.clone();
        let domain = url.domain().expect("No host.");
        let split = domain.split('.').collect::<Vec<_>>();
        let l = split.len();
        Host::parse(split[l.saturating_sub(2)..l].join(".").as_str()).unwrap()
    }

    /// Free most of a Page
    pub fn empty(&mut self) { *self.html.lock().unwrap() = None; }

    /// Freshness check
    pub fn is_old(&mut self) -> bool {
        Utc::now() - *self.last.lock().unwrap() > Duration::minutes(10)
    }

    /// Get the chapter,page number (and index?)
    pub fn num(&self, find: Find<'_>) -> (u16, u16, String) { find.num(self) }

    /// Get the title with a Finder
    pub async fn title(&self, find: Find<'_>) -> Label {
        self.html
            .lock()
            .unwrap()
            .as_ref()
            .map(|d| find.title(d))
            .unwrap_or_default()
    }

    /// Returns a Page leading the the index page of the chapter
    pub async fn index(&self, find: Find<'_>) -> Option<Self> {
        let res = self.html.lock().unwrap().as_ref().map(|d| {
            {
                find.index(d).unwrap_or({
                    // TODO: Alternatively, find links up or left from other
                    // links leading to the current
                    // page
                    let base = self.url.origin().ascii_serialization();
                    let mut index = self
                        .url
                        .path_segments()
                        .unwrap()
                        .rev()
                        .fold((Vec::new(), 0, 0), |mut acc, s| {
                            if s.to_lowercase().contains("chapter") {
                                acc.1 += 1;
                            } else if acc.1 != 0 || acc.2 > 1 {
                                acc.0.push(s);
                            }
                            acc.2 += 1;
                            acc
                        })
                        .0;
                    index.push(&base);
                    index
                        .iter()
                        .rev()
                        .copied()
                        .collect::<Vec<_>>()
                        .join("/")
                        .parse()
                        .expect("Couldn't resolve Index.")
                })
            }
        });
        res
    }

    /// Get the links from the lowest div with most links
    pub async fn links(&self, find: Find<'_>) -> Vec<Page> {
        self.html
            .lock()
            .unwrap()
            .as_ref()
            .map(|d| find.links(d))
            .unwrap_or_default()
    }

    /// Get next page
    pub async fn next(&self, find: Find<'_>) -> Option<Page> {
        self.html
            .lock()
            .unwrap()
            .as_ref()
            .and_then(|d| find.next(d))
    }

    /// Get text chapter
    pub async fn text(&self, find: Find<'_>) -> Vec<String> {
        self.html
            .lock()
            .unwrap()
            .as_ref()
            .map(|d| find.text(d))
            .unwrap_or_default()
    }

    /// Get pages to images
    pub fn images(&self, find: Find<'_>) -> Vec<Page> {
        self.html
            .lock()
            .unwrap()
            .as_ref()
            .map(|d| find.images(d))
            .unwrap_or_default()
    }

    /// Download a single image from a Page with an url leading to an image
    pub async fn image(&self, client: &Client) -> Vec<u8> {
        //        let mut res = client
        //
        // .execute(self.req.as_ref().as_ref().unwrap().try_clone().unwrap())
        //            .await
        //            .unwrap();
        //        let mut ch = vec![];
        //        while let Some(chunk) = res.chunk().await.unwrap() {
        //            ch.append(&mut chunk.to_vec());
        //        }
        //        Box::new(ch)

        client
            .execute(self.req.as_ref().unwrap().try_clone().unwrap())
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap()
            .to_vec()
    }

    /// Get a text chapter
    pub async fn novel(&self, find: Find<'_>) -> Box<Vec<u8>> {
        Box::new(
            self.html
                .lock()
                .unwrap()
                .as_ref()
                .map(|d| find.text(d).join("\n\n").bytes().collect())
                .unwrap_or_default(),
        )
    }
}

impl Eq for Page {}
impl PartialEq for Page {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url &&
            *self.html.lock().unwrap() == *other.html.lock().unwrap() &&
            *self.last.lock().unwrap() == *other.last.lock().unwrap()
    }
}
impl Ord for Page {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (&self.url, *self.last.lock().unwrap())
            .cmp(&(&other.url, *other.last.lock().unwrap()))
    }
}
impl PartialOrd for Page {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            (&self.url, *self.last.lock().unwrap())
                .cmp(&(&other.url, *other.last.lock().unwrap())),
        )
    }
}
impl Default for Page {
    fn default() -> Self {
        Self {
            url:  "http://codenova.ddns.net".parse().unwrap(),
            req:  Default::default(),
            html: Default::default(),
            last: Arc::new(Mutex::new(Utc::now() - Duration::days(1))),
        }
    }
}
impl Clone for Page {
    fn clone(&self) -> Self {
        Self {
            url:  self.url.clone(),
            req:  self.req.as_ref().map(|x| x.try_clone().unwrap()),
            html: self.html.clone(),
            last: self.last.clone(),
        }
    }
}
impl Hash for Page {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.url.hash(state);
        self.html.lock().unwrap().hash(state);
        self.last.lock().unwrap().hash(state);
    }
}

impl<T: Into<String>> From<T> for Page {
    fn from(s: T) -> Self {
        let mut ok = Self::default();
        if let Ok(u) = s.into().parse::<Url>() {
            ok.url = u
        }
        ok
    }
}
impl FromStr for Page {
    type Err = url::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            url: s.parse()?,
            ..Default::default()
        })
    }
}
