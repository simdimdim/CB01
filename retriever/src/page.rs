use crate::{extractor::Extractor, Index, Links, Next, Title};
use log::{debug, info, trace};
use reqwest::{
    header::{HeaderValue, REFERER},
    Client,
    Url,
};
use select::document::Document;
use std::{
    borrow::Cow,
    convert::TryFrom,
    fmt::Debug,
    ops::Deref,
    path::Path,
    str::FromStr,
    time::Duration,
};
use time::OffsetDateTime;
use tokio::{fs::write, io};
use url::ParseError;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Page<T = fn() -> String> {
    pub url: Url,
    pub next_by: SepStr,
    pub split_by: T,
    pub html: Option<String>,
    pub content: Content,
    pub referer: Option<HeaderValue>,
    pub last: Option<OffsetDateTime>,
}
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub enum ContentType {
    Text(Vec<String>, Option<String>),
    Image(Vec<u8>),
    Chapter(Vec<ContentType>),
    Images(Vec<String>, Option<String>),
    Chapters(Vec<String>, Option<String>),
    #[default]
    Empty,
}
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct Content {
    name: Title,
    index: Index,
    next: Next,
    links: Links,
    pub data: Option<ContentType>,
}

impl Page {
    pub fn url(&mut self, url: Url) -> &mut Self {
        trace!(
            "Set page url to: {} from:\n{}",
            url.as_str(),
            self.url.as_str()
        );
        self.url = url;
        self
    }

    pub async fn visit(
        &mut self, client: Client, extractor: &Extractor, visual: bool,
    ) -> &mut Self {
        info!("Visited: {}", self.url.as_str());
        let req = client
            .get(self.url.as_ref())
            .header(
                REFERER,
                self.referer
                    .as_ref()
                    .unwrap_or(&HeaderValue::from_str("").unwrap()),
            )
            .build()
            .unwrap_or_else(|_| panic!("Failed to build request for: {}", &self.url));
        let page = client
            .execute(req.try_clone().unwrap())
            .await
            .expect("Failed to unwrap response");
        self.last = Some(OffsetDateTime::now_utc());
        if let Some(ContentType::Image(ref mut data)) = self.content.data {
            let mut bytes = page.bytes().await;
            // Retry once
            if bytes.is_err() {
                tokio::time::sleep(Duration::from_millis(2000)).await;
                let page = client
                    .execute(req)
                    .await
                    .expect("Failed to unwrap response");
                self.last = Some(OffsetDateTime::now_utc());
                bytes = page.bytes().await;
            }
            *data = bytes.unwrap().to_vec();
            trace!("Early return, Image");
            return self;
        };
        self.html = Some(page.text().await.expect("Failed to get html source code"));
        trace!("html: {:?}", &self.html);
        self.content.name = extractor.get_title(self).await;
        trace!("name: {:?}", &self.content.name);
        self.content.index = extractor.get_index(self).await;
        trace!("index: {:?}", &self.content.index);
        self.content.next = extractor.get_next(self).await;
        trace!("next: {:?}", &self.content.next);
        self.content.links = extractor.get_links(self).await;
        trace!("links: {:?}", &self.content.links.as_ref().map(|l| l.len()));
        trace!("links: {:?}", &self.content.links.as_ref().map(|l| &l[..2]));
        self.content.data = if visual {
            extractor.get_images(self).await
        } else {
            extractor.get_text(self).await
        };
        trace!("data: {:?}", &self.content.data);
        // TODO: convert and assign to Content
        self.empty();
        self
    }

    pub fn doc(&self) -> Option<Document> { self.html.as_ref().map(|s| Document::from(s.as_str())) }

    pub fn name(&self) -> &str {
        //TODO name from url/title else path()
        self.url.path_segments().unwrap().last().unwrap()
    }

    pub fn chapter(&self) -> &str {
        //TODO name from url/title else path()
        //TODO symmetric difference/intersection with next page url
        let res = self
            .url
            .path_segments()
            .unwrap()
            .rev()
            .skip_while(|s| s.is_empty())
            .nth(1)
            .unwrap();
        debug!("url final segment is: {:?}", &res);
        res
    }

    pub fn path(&self) -> &str { self.url.path() }

    pub fn filename(&self) -> Option<String> { self.url.path_segments()?.last().map(str::to_owned) }

    pub fn origin(&self) -> String { self.url.origin().unicode_serialization() }

    pub fn domain(&self) -> Option<&str> { self.url.domain() }

    pub fn host(&self) -> Option<String> { self.url.host_str().map(str::to_owned) }

    pub fn get_next(&self) -> &str { self.next_by.deref() }

    pub fn set_next(&mut self, n: SepStr) -> &mut Self {
        self.next_by = n;
        self
    }

    pub fn split(&self) -> String { (self.split_by)() }

    pub fn content(&self) -> &Content { &self.content }

    pub async fn save(&self, pb: &Path) -> io::Result<()> {
        let final_path = pb.join(self.filename().as_deref().unwrap_or_else(|| self.chapter()));
        debug!("base path {:?}", &final_path);
        self.content.save(&final_path).await
    }

    pub fn empty(&mut self) { self.html = None; }
}
impl Content {
    pub fn name(&self) -> Option<&String> { self.name.as_ref() }

    pub fn rename(&mut self, name: String) -> &mut Self {
        self.name.replace(name);
        self
    }

    pub fn index(&self) -> &Index { &self.index }

    pub fn next(&self) -> &Next { &self.next }

    pub fn links(&self) -> &Links { &self.links }

    pub async fn save(&self, pb: &Path) -> io::Result<()> {
        let name_from = |cnt: &[u8]| -> String {
            let res = self
                .name
                .as_ref()
                .unwrap_or(&Uuid::new_v5(&Uuid::NAMESPACE_OID, cnt).to_string())
                .to_owned();
            debug!("generated new name: {}", res);
            res
        };
        trace!("data is: {:?}", &self.data);
        match &self.data {
            Some(data @ ContentType::Text(..)) => {
                let z = pb.to_path_buf();
                trace!("path is: {:?}", &z);
                let contents = data.as_data();
                // z = z.join(name_from(&contents));
                // let p = pb.join(name_from(&contents[..]));
                // trace!("final text path: {:?}", p);
                write(z, contents).await?;
            }
            Some(ContentType::Image(data)) => {
                let mut z = pb.to_path_buf();
                z.pop();
                std::fs::create_dir_all(&z).expect("Failed to create path to output directory.");
                let file = name_from(data);
                let mut filename = pb.file_name().unwrap_or_default().to_os_string();
                let f = file
                    .to_ascii_lowercase()
                    .chars()
                    .zip(
                        filename
                            .clone()
                            .into_string()
                            .unwrap_or_default()
                            .to_ascii_lowercase()
                            .chars()
                            .chain(std::iter::repeat('*')),
                    )
                    .filter(|(a, b)| a != b)
                    .map(|(a, _)| a)
                    .collect::<String>()
                    .replace('-', "_");
                filename.push(&f);
                let mut pb = pb.to_path_buf();
                pb.set_file_name(filename);
                debug!("final image path: {:?}", &pb);
                write(pb, data).await?;
            }
            _ => (),
            // Some(ContentType::Images(data, _)) => {
            //     let contents = data[..].join("\n");
            //     let mut p = pb.join(name_from(contents.as_bytes())).join("sources");
            //     p.set_extension(".lst");
            //     write(p, contents).await?;
            // }
            // Some(ContentType::Chapters(data)) => {
            //     let contents = data[..].join("\n");
            //     let mut p = pb.join(name_from(contents.as_bytes())).join("sources");
            //     p.set_extension(".lst");
            //     write(p, contents).await?;
            // }
        };
        Ok(())
    }
}
impl ContentType {
    pub fn as_data(&self) -> Cow<'_, [u8]> {
        match self {
            ContentType::Text(data, delim) => {
                trace!("as_data text");
                data.join(delim.as_deref().unwrap_or("\n"))
                    .into_bytes()
                    .into()
            }
            ContentType::Image(data) => data.into(),
            // ContentType::Chapter(_) => todo!(),
            // ContentType::Images(_, _) => todo!(),
            // ContentType::Chapters(_) => todo!(),
            // ContentType::Empty => todo!(),
            _ => Cow::default(),
        }
    }

    pub fn to_pages(&self) -> Option<Vec<Page>> {
        fn convert(input: &[String], host: &Option<String>) -> Vec<Page> {
            debug!("host was: {:?}", host);
            input
                .iter()
                .zip(std::iter::repeat(host))
                .filter_map(|(p, h)| {
                    let pa = (h
                        .as_ref()
                        .map(|s| s.clone() + p)
                        .unwrap_or_else(|| p.clone()))
                    .parse()
                    .ok();
                    pa.map(|mut q: Page| {
                        q.content.name = q.filename();
                        q
                    })
                })
                .collect()
        }
        match self {
            ContentType::Images(urls, referer) => {
                let referer = referer.as_ref().map(|r| r.try_into().unwrap());
                Some(
                    convert(urls, &None)
                        .into_iter()
                        .map(|mut p| {
                            p.referer = referer.clone();
                            p.content = Content {
                                name: p.filename(),
                                data: Some(ContentType::Image(vec![])),
                                ..Default::default()
                            };
                            p
                        })
                        .collect(),
                )
            }
            ContentType::Chapters(urls, host) => Some(convert(urls, host)),
            _ => None,
        }
    }
}

impl From<Vec<u8>> for Content {
    fn from(data: Vec<u8>) -> Self {
        Self {
            name: None,
            index: None,
            next: None,
            links: None,
            data: Some(ContentType::Image(data)),
        }
    }
}
impl From<ContentType> for Content {
    fn from(data: ContentType) -> Self {
        Self {
            data: Some(data),
            ..Default::default()
        }
    }
}

impl Default for Page {
    fn default() -> Self {
        Self {
            url: "http://codenova.duckdns.org/app".parse().unwrap(),
            next_by: Default::default(),
            split_by: || " Chapter".to_owned(),
            html: Default::default(),
            content: Default::default(),
            referer: Default::default(),
            last: Default::default(),
        }
    }
}
impl Iterator for Page {
    type Item = Page;

    fn next(&mut self) -> Option<Self::Item> {
        let s = self.content.next.as_ref()?; // .and_then(|s| s.try_into().ok())
        let out: Option<Page> = s.try_into().ok();
        if out.is_none() {
            return (self.origin() + s).try_into().ok();
        };
        out
    }
}
impl FromStr for Page {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            url: s.parse()?,
            ..Default::default()
        })
    }
}
impl TryFrom<String> for Page {
    type Error = ParseError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(Self {
            url: s.parse::<Url>()?,
            ..Default::default()
        })
    }
}
impl TryFrom<&String> for Page {
    type Error = ParseError;

    fn try_from(s: &String) -> Result<Self, Self::Error> {
        Ok(Self {
            url: s.parse::<Url>()?,
            ..Default::default()
        })
    }
}
impl TryFrom<&mut String> for Page {
    type Error = ParseError;

    fn try_from(s: &mut String) -> Result<Self, Self::Error> {
        Ok(Self {
            url: s.parse::<Url>()?,
            ..Default::default()
        })
    }
}
impl From<Url> for Page {
    fn from(url: Url) -> Self {
        Self {
            url,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub enum SepStr {
    #[default]
    Next,
    NEXT,
    Arrow,
    Custom(&'static str),
}
impl From<&'static str> for SepStr {
    fn from(s: &'static str) -> Self {
        match s {
            "Next" => SepStr::Next,
            "NEXT" => SepStr::NEXT,
            "->" => SepStr::Arrow,
            x => Self::Custom(x),
        }
    }
}
impl Deref for SepStr {
    type Target = &'static str;

    fn deref(&self) -> &Self::Target {
        match self {
            SepStr::Next => &"Next",
            SepStr::NEXT => &"NEXT",
            SepStr::Arrow => &"->",
            SepStr::Custom(s) => s,
        }
    }
}
