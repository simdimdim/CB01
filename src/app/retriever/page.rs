use crate::{DefaultFinder, Finder, Label};
use chrono::{DateTime, Duration, Utc};
use reqwest::{Request, Url};
use select::document::Document;
use std::{
    hash::{Hash, Hasher},
    str::FromStr,
};
use url::Host;

#[derive(Debug)]
pub struct Page {
    pub url:  Url,
    pub req:  Option<Request>,
    pub html: Option<String>,
    pub doc:  Option<Document>,
    pub last: DateTime<Utc>,
    pub full: bool,
}

impl Page {
    pub fn prep(&mut self, re: Request) -> &mut Self {
        self.req = Some(re);
        self.full = false;
        self
    }

    /// Get the `example.com` from `http://example.com/path/`
    /// would fail for http://localhost/path
    pub fn domain(&self) -> Result<Host, String> {
        match self.url.host() {
            Some(d) => Ok(d.to_owned()),
            _ => Err("No host.".to_owned()),
        }
    }

    ///Lighten a Page
    pub fn empty(&mut self) {
        self.html = None;
        self.doc = None;
    }

    /// Fullness check
    pub fn is_empty(&mut self) -> bool { self.html != None }

    /// Returns a Page leading the the index page of the chapter
    pub fn index(&self) -> Result<Self, url::ParseError> {
        // TODO: Alternatively, find links up or left from other links leading to
        // the current page
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
    }

    /// Get the title with an optional Finder
    pub fn title<T: Finder>(&self, finder: Option<T>) -> Label {
        match finder {
            Some(f) => f.title(&self.doc),
            None => DefaultFinder.title(&self.doc),
        }
    }

    pub fn links<T: Finder>(&self, finder: Option<T>) -> Vec<Page> {
        match finder {
            Some(f) => f.links(&self.doc),
            None => DefaultFinder.links(&self.doc),
        }
    }

    pub fn next<T: Finder>(&self, finder: Option<T>) -> Option<Page> {
        match finder {
            Some(f) => f.next(&self.doc),
            None => DefaultFinder.next(&self.doc),
        }
    }

    pub fn text<T: Finder>(&self, finder: Option<T>) -> Vec<String> {
        match finder {
            Some(f) => f.text(&self.doc),
            None => DefaultFinder.text(&self.doc),
        }
    }

    pub fn images<T: Finder>(&self, finder: Option<T>) -> Vec<Page> {
        match finder {
            Some(f) => f.images(&self.doc),
            None => DefaultFinder.images(&self.doc),
        }
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
            full: self.full,
        }
    }
}
impl Default for Page {
    fn default() -> Self {
        Self {
            url:  "http://codenova.ddns.net".parse().unwrap(),
            req:  None,
            html: None,
            doc:  None,
            last: Utc::now() - Duration::days(1),
            full: false,
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
        self.html.hash(state);
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
