use crate::{Finder, Label};
use chrono::{DateTime, Duration, Utc};
use reqwest::{Client, Request, Url};
use select::document::Document;
use std::{
    cell::RefCell,
    hash::{Hash, Hasher},
    str::FromStr,
};
use url::Host;

#[derive(Debug)]
pub struct Page {
    pub url:  Url,
    pub req:  Option<Request>,
    pub html: RefCell<Option<String>>,
    pub doc:  RefCell<Option<Document>>,
    pub last: DateTime<Utc>,
}

type Find<'a> = &'a Box<dyn Finder>;

impl Page {
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
        let html = resp.text().await.unwrap();
        self.doc.replace(Some(html.as_str().into()));
        self.html.replace(Some(html));
        self.to_owned()
    }

    /// Loads the html and parsed html in Page preparation for future actions
    pub async fn fetch_solo(&self) -> Self {
        // TODO: Better error recovery with a failures counter in Retriever
        let resp = Client::new()
            .execute(self.req.as_ref().unwrap().try_clone().unwrap())
            .await
            .unwrap();
        let html = resp.text().await.unwrap();
        self.doc.replace(Some(html.as_str().into()));
        self.html.replace(Some(html));
        self.to_owned()
    }

    /// Get the `example.com` from `http://example.com/path/`
    /// would fail for http://localhost/path
    pub fn domain(&self) -> Host {
        self.url.host().map(|s| s.to_owned()).expect("No host.")
    }

    /// Free most of a Page
    pub fn empty(&mut self) {
        self.html.replace(None);
        self.doc.replace(None);
    }

    /// Freshness check
    pub fn is_old(&mut self) -> bool {
        Utc::now() - self.last > Duration::minutes(10)
    }

    /// Get the chapter,page number (and index?)
    pub fn num(&self, find: Find) -> (u16, u16, String) { find.num(self) }

    /// Get the title with a Finder
    pub fn title(&self, find: Find) -> Label {
        self.doc
            .borrow()
            .as_ref()
            .map(|d| find.title(d))
            .unwrap_or_default()
    }

    /// Returns a Page leading the the index page of the chapter
    pub fn index(&self, find: Find) -> Option<Self> {
        let res = self.doc.borrow().as_ref().map(|d| {
            find.index(d).unwrap_or({
                // TODO: Alternatively, find links up or left from other links
                // leading to the current page
                let base = self.url.origin().ascii_serialization();
                let mut index = self
                    .url
                    .path_segments()
                    .unwrap()
                    .rev()
                    .fold((Vec::new(), 0, 0), |mut acc, s| {
                        if s.to_lowercase().contains("chapter") {
                            acc.1 += 1;
                        } else {
                            if acc.1 != 0 || acc.2 > 1 {
                                acc.0.push(s);
                            }
                        }
                        acc.2 += 1;
                        acc
                    })
                    .0;
                index.push(&base);
                index
                    .iter()
                    .rev()
                    .map(|&a| a)
                    .collect::<Vec<_>>()
                    .join("/")
                    .parse()
                    .expect("Couldn't resolve Index.")
            })
        });
        res
    }

    /// Get the links from the lowest div with most links
    pub fn links(&self, find: Find) -> Vec<Page> {
        self.doc
            .borrow()
            .as_ref()
            .map(|d| find.links(d))
            .unwrap_or_default()
    }

    /// Get next page
    pub fn next(&self, find: Find) -> Option<Page> {
        self.doc.borrow().as_ref().and_then(|d| find.next(d))
    }

    /// Get text chapter
    pub fn text(&self, find: Find) -> Vec<String> {
        self.doc
            .borrow()
            .as_ref()
            .map(|d| find.text(d))
            .unwrap_or_default()
    }

    /// Get pages to images
    pub fn images(&self, find: Find) -> Vec<Page> {
        self.doc
            .borrow()
            .as_ref()
            .map(|d| find.images(d))
            .unwrap_or_default()
    }

    pub async fn image(&self, client: &Client) -> Box<Vec<u8>> {
        Box::new(
            client
                .execute(self.req.as_ref().unwrap().try_clone().unwrap())
                .await
                .unwrap()
                .bytes()
                .await
                .unwrap()
                .to_vec(),
        )
    }

    pub fn novel(&self, find: Find<'_>) -> Box<Vec<u8>> {
        Box::new(
            self.doc
                .borrow()
                .as_ref()
                .map(|d| find.text(d).join("\n\n").bytes().collect::<Vec<_>>())
                .unwrap_or_default(),
        )
    }
}

impl Clone for Page {
    fn clone(&self) -> Self {
        Self {
            url:  self.url.clone(),
            req:  self.req.as_ref().map(|x| x.try_clone().unwrap()),
            html: self.html.clone(),
            doc:  self.doc.clone(),
            last: self.last.clone(),
        }
    }
}
impl Default for Page {
    fn default() -> Self {
        Self {
            url:  "http://codenova.ddns.net".parse().unwrap(),
            req:  None,
            html: RefCell::new(None),
            doc:  RefCell::new(None),
            last: Utc::now() - Duration::days(1),
        }
    }
}
impl Eq for Page {}
impl PartialEq for Page {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url &&
            self.html == other.html &&
            self.last == other.last
    }
}
impl Ord for Page {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (&self.url, self.last).cmp(&(&other.url, other.last))
    }
}
impl PartialOrd for Page {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((&self.url, self.last).cmp(&(&other.url, other.last)))
    }
}
impl Hash for Page {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.url.hash(state);
        self.html.borrow().hash(state);
        self.last.hash(state);
    }
}

impl<T: Into<String>> From<T> for Page {
    fn from(s: T) -> Self {
        let mut ok = Self::default();
        match s.into().parse::<Url>() {
            Ok(u) => ok.url = u,
            Err(_) => (),
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
