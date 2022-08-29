// use cached::proc_macro::cached;
use chrono::{Date, DateTime, Utc};
use reqwest::{Client, Request};
use std::str::FromStr;
use url::{Host, Url};

use crate::extractors::DogHouse;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Source {
    url: Url,
    extractor: Option<usize>, // (extractor type, extractor number)
}
#[derive(Debug)]
pub struct Page {
    pub contents: Option<String>,
    pub req: Option<Request>,
    pub last: Option<Date<Utc>>,
}

impl Source {
    pub fn domain(&self) -> &Host {
        match self.url.domain() {
            Some(_) => todo!(),
            None => todo!(),
        }
    }

    pub fn path(&self) -> &str { self.url.path() }

    pub fn url(&self) -> &Url { &self.url }

    pub async fn contents(&self) -> reqwest::Result<Page> {
        match Page::get(self).await {
            Ok(_) => todo!(),
            Err(_) => todo!(),
        }
    }

    pub async fn next(&self, dh: &DogHouse) -> Option<Source> {
        if let Some(n) = self.extractor {
            dh.next(self.contents().await.unwrap().html(), n, None)
                .expect("Couldn't get extractor");
        }
        None
    }
}
impl Page {
    // #[cached]
    async fn get(source: &Source) -> reqwest::Result<(Page, DateTime<Utc>)> {
        let _res = reqwest::get(source.url().to_owned()).await?;
        Ok((
            Page {
                contents: None,
                req: None,
                last: None,
            },
            Utc::now(),
        ))
    }

    pub async fn fetch(&mut self, client: &Client) -> &mut Self {
        let resp = client
            .execute(self.req.as_ref().unwrap().try_clone().unwrap())
            .await
            .unwrap();
        self.contents = Some(resp.text().await.unwrap());
        self
    }

    pub fn html(&self) -> String { "".to_owned() }
}
// use reqwest::header::HeaderMap;
// headers: Option<HeaderMap>,
// pub fn headers() -> HeaderMap {
//     self.headers.unwrap_or_else(|| {
//         let mut hm = HeaderMap::new();
//         hm.insert(
//             reqwest::header::REFERER,
//             "https://readmanganato.com/".parse().unwrap(),
//         );
//         hm
//     })
// }
impl Eq for Page {}
impl PartialEq for Page {
    fn eq(&self, other: &Self) -> bool {
        self.contents == other.contents && self.last == other.last
    }
}
impl Clone for Page {
    fn clone(&self) -> Self {
        Self {
            contents: self.contents.clone(),
            req: self.req.as_ref().map(|x| x.try_clone().unwrap()),
            last: self.last,
        }
    }
}
impl Default for Source {
    fn default() -> Self {
        Self {
            url: "http://codenova.ddns.net/app".parse().unwrap(),
            extractor: None,
        }
    }
}
impl<T: Into<String>> From<T> for Source {
    fn from(s: T) -> Self {
        let mut ok = Self::default();
        if let Ok(u) = s.into().parse::<Url>() {
            ok.url = u
        }
        ok
    }
}
impl FromStr for Source {
    type Err = url::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            url: s.parse()?,
            ..Default::default()
        })
    }
}
