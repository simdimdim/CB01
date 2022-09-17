#![feature(iter_advance_by)]
use clap::Parser;
use futures::future::join_all;
use log::{debug, info, trace};
use reqwest::{
    header::{HeaderValue, REFERER},
    Client,
    Url,
};
use select::{
    document::Document,
    predicate::{Any, Child, Descendant, Name, Or, Text as Txt},
};
use static_init::dynamic;
use std::{
    borrow::Cow,
    convert::{TryFrom, TryInto},
    fmt::Debug,
    io,
    path::PathBuf,
    str::FromStr,
    time::Duration,
};
use time::OffsetDateTime;
use tokio::fs::write;
use url::ParseError;
use uuid::Uuid;

type Title = Option<String>;
type Index = Option<String>;
type Next = Option<String>;
type Links = Option<Vec<String>>;
type Text = Option<ContentType>;
type Images = Option<ContentType>;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Opt {
    #[clap(short, long, value_parser)]
    /// String contained in the next page button
    next: Option<String>,
    #[clap(short, long, value_parser)]
    /// String to split the title by to get novel/manga title
    split: Option<String>,
    #[clap(short, long, value_parser)]
    /// Output directory
    out_dir: Option<PathBuf>,
    #[clap(
        short,
        long = "manga",
        required(true),
        group = "kind",
        conflicts_with = "novel"
    )]
    /// Grab images
    image: bool,
    #[clap(short = 't', long, required(true), group = "kind")]
    /// Grab text
    novel: bool,
    #[clap(min_values(1))]
    /// url/s to manga or novels
    urls: Vec<Url>,
    #[clap(short, long, value_parser, default_value = "400")]
    /// How long to sleep [in milliseconds] between requesting pages
    delay: u64,
    #[clap(short, long, group = "text")]
    /// Set dedicated text extractor for RR
    royalroad: bool,
}
#[derive(Clone)]
pub struct Extractor {
    title: Option<fn(&Page) -> Title>,
    index: Option<fn(&Page) -> Index>,
    next: Option<fn(&Page) -> Next>,
    links: Option<fn(&Page) -> Links>,
    text: Option<fn(&Page) -> Text>,
    images: Option<fn(&Page) -> Images>,
}
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub enum ContentType {
    Text(Vec<String>, Option<String>),
    Image(Vec<u8>),
    Chapter(Vec<ContentType>),
    Images(Vec<String>, Option<String>),
    Chapters(Vec<String>),
    #[default]
    Empty,
}
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct Content {
    name: Title,
    index: Index,
    next: Next,
    links: Links,
    data: Option<ContentType>,
}
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Page<T = fn() -> String> {
    url: Url,
    pub next_by: T,
    pub split_by: T,
    html: Option<String>,
    content: Content,
    referer: Option<HeaderValue>,
    last: Option<OffsetDateTime>,
}

#[dynamic(lazy)]
static mut NEXT: &'static str = "A";
#[allow(unreachable_code)]
#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::init();

    let args = Opt::parse();
    let mut extractor = Extractor::default();
    if args.royalroad {
        extractor.text = Some(rr_text);
    }
    if let Some(dir) = &args.out_dir {
        std::fs::create_dir_all(dir).expect("Failed to create path to output directory.");
    }
    if let Some(next) = args.next {
        info!("Set string to look for next chapter link to: \"{}\"", &next);
        let mut s = NEXT.write();
        *s = Box::leak(next.into_boxed_str());
    }
    if let Some(_split) = args.split {}
    info!("delay: {}", &args.delay);
    info!("Looking for {}", if args.image { "images" } else { "text" });
    let mut content = vec![];
    let mut page = args.urls.into_iter().map(Into::into).into_iter().next();
    // fetch(pages.as_mut(), &extractor, args.text).await;
    while let Some(mut p) = page {
        trace!("Delayed: {}", &args.delay);
        fetch_one(&mut p, &extractor, args.image).await;
        if let Some(cnt) = p.content.data.take() {
            let path = args.out_dir.clone().unwrap();
            debug!("path base: {:?}", path);
            match cnt {
                ContentType::Text(_, _) => {
                    let final_path = path.clone().join(p.chapter());
                    debug!("path with page name: {:?}", final_path);
                    p.content.data = Some(cnt);
                    p.content.save(final_path).await.unwrap();
                }
                images @ ContentType::Images(_, _) => {
                    if let Some(mut res) = images.to_pages() {
                        fetch(&mut res, &extractor, args.image).await;
                        join_all(res.iter().map(|c| async {
                            let final_path = path.clone().join(p.chapter());
                            debug!("path with chapter: {:?}", final_path);
                            c.content.save(final_path).await
                        }))
                        .await;
                        content.push(res);
                    }
                }
                _ => {}
            };
        };
        page = p.next();
        tokio::time::sleep(Duration::from_millis(args.delay)).await;
    }
    debug!("Gathered {:?} pages", content);
    Ok(())
}

impl ContentType {
    pub fn as_data(&self) -> Cow<'_, [u8]> {
        match self {
            ContentType::Text(data, delim) => {
                debug!("as_data text");
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

    pub fn to_pages(self) -> Option<Vec<Page>> {
        fn convert(input: Vec<String>) -> Vec<Page> {
            input.into_iter().filter_map(|p| p.parse().ok()).collect()
        }
        match self {
            ContentType::Images(urls, referer) => {
                let referer = referer.as_ref().map(|r| r.try_into().unwrap());
                Some(
                    convert(urls)
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
            ContentType::Chapters(urls) => Some(convert(urls)),
            _ => None,
        }
    }
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
        &mut self, client: Option<Client>, extractor: &Extractor, visual: bool,
    ) -> &mut Self {
        info!("Visited: {}", self.url.as_str());
        let client = client.unwrap_or_else(Client::new);
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
            .execute(req)
            .await
            .expect("Failed to unwrap response");
        self.last = Some(OffsetDateTime::now_utc());
        if let Some(ContentType::Image(ref mut data)) = self.content.data {
            *data = page.bytes().await.unwrap().to_vec();
            trace!("Early return, Image");
            return self;
        };
        self.html = Some(page.text().await.expect("Failed to get html source code"));
        self.content.name = extractor.title(self).await;
        self.content.index = extractor.index(self).await;
        self.content.next = extractor.next(self).await;
        self.content.links = extractor.links(self).await;
        self.content.data = if visual {
            extractor.images(self).await
        } else {
            extractor.text(self).await
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
        let mut res = self.url.path_segments().unwrap();
        if self.url.path().ends_with('/') {
            res.advance_back_by(1).unwrap();
        }
        let last = res.last().unwrap();
        debug!("url final segment is: {:?}", &last);
        last
    }

    pub fn path(&self) -> &str { self.url.path() }

    pub fn filename(&self) -> Option<String> { self.url.path_segments()?.last().map(str::to_owned) }

    pub fn origin(&self) -> String { self.url.origin().unicode_serialization() }

    pub fn domain(&self) -> Option<&str> { self.url.domain() }

    pub fn with_host(mut self, host: String) -> Self {
        self.url = (host + self.url.as_str()).parse().unwrap();
        debug!("You're here: {:?}", &self.url);
        self
    }

    pub fn host(&self) -> Option<&str> { self.url.host_str() }

    pub fn pred(&self) -> String {
        // (self.next)()
        NEXT.fast_read().unwrap().to_string()
    }

    pub fn split(&self) -> String { (self.split_by)() }

    pub fn content(&self) -> &Content { &self.content }

    pub fn empty(&mut self) { self.html = None; }
}
impl Extractor {
    pub fn new() -> Self {
        Self {
            title: None,
            next: None,
            index: None,
            links: None,
            text: None,
            images: None,
        }
    }

    pub async fn title(&self, page: &Page) -> Title { self.title.and_then(|f| f(page)) }

    pub async fn next(&self, page: &Page) -> Next { self.next.and_then(|f| f(page)) }

    pub async fn index(&self, page: &Page) -> Index { self.index.and_then(|f| f(page)) }

    pub async fn links(&self, page: &Page) -> Links { self.links.and_then(|f| f(page)) }

    pub async fn text(&self, page: &Page) -> Text { self.text.and_then(|f| f(page)) }

    pub async fn images(&self, page: &Page) -> Images { self.images.and_then(|f| f(page)) }
}
impl Content {
    pub fn name(&self) -> Option<&String> { self.name.as_ref() }

    pub fn rename(&mut self, name: String) -> &mut Self {
        self.name.replace(name);
        self
    }

    pub async fn save(&self, pb: PathBuf) -> io::Result<()> {
        let name_from = |cnt: &[u8]| -> String {
            let res = self
                .name
                .as_ref()
                .unwrap_or(&Uuid::new_v5(&Uuid::NAMESPACE_OID, cnt).to_string())
                .to_owned();
            debug!("generated new name: {}", res);
            res
        };
        debug!("data is: {:?}", &self.data);
        match &self.data {
            Some(data @ ContentType::Text(..)) => {
                debug!("path is: {:?}", &pb);
                let contents = data.as_data();
                // let p = pb.join(name_from(&contents[..]));
                // debug!("final text path: {:?}", p);
                write(pb, contents).await?;
            }
            Some(ContentType::Image(data)) => {
                std::fs::create_dir_all(&pb).expect("Failed to create path to output directory.");
                let p = pb.join(name_from(data));
                debug!("final image path: {:?}", p);
                write(p, data).await?;
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

impl Debug for Extractor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::any::type_name;
        fn type_name_of<T>(_: T) -> &'static str { type_name::<T>() }
        f.debug_struct("Extractor")
            .field("title", &self.title.map(|f| type_name_of(f)))
            .field("next", &self.next.map(|f| type_name_of(f)))
            .field("index", &self.index.map(|f| type_name_of(f)))
            .field("links", &self.links.map(|f| type_name_of(f)))
            .field("text", &self.text.map(|f| type_name_of(f)))
            .field("images", &self.images.map(|f| type_name_of(f)))
            .finish()
    }
}
impl Default for Extractor {
    fn default() -> Self {
        Self {
            title: Some(default_title),
            next: Some(default_next),
            index: Some(default_index),
            links: Some(default_links),
            text: Some(default_text),
            images: Some(default_images),
        }
    }
}
impl Default for Page {
    fn default() -> Self {
        Self {
            url: "http://codenova.duckdns.org/app".parse().unwrap(),
            next_by: || "Next".to_owned(),
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

pub async fn fetch(pages: &mut [Page], extractor: &Extractor, visual: bool) {
    join_all(pages.iter_mut().map(|p| async {
        fetch_one(p, extractor, visual).await;
        p
    }))
    .await;
}
pub async fn fetch_one(page: &mut Page, extractor: &Extractor, visual: bool) {
    if let Some(time) = page.last {
        if time.minute() < 1 {
            info!("Visited recently");
            return;
        }
    }
    page.visit(None, extractor, visual).await;
}
pub fn default_title(page: &Page) -> Title {
    page.doc().map(|d| {
        let title = d
            .select(Name("title"))
            .into_selection()
            .first()
            .unwrap()
            .text();
        if title.contains(page.split().as_str()) {
            title
                .split(page.split().as_str())
                .filter(|&a| !a.is_empty())
                .collect::<Vec<_>>()
                .first()
                .unwrap()
                .to_string()
        } else {
            title
        }
    })
}
pub fn default_next(page: &Page) -> Next {
    page.doc().and_then(|d| {
        d.select(Child(Name("a"), Txt))
            .filter(|a| a.text().contains(page.pred().as_str()))
            .map(|a| a.parent().unwrap().attr("href").unwrap().to_string())
            .next()
    })
}
pub fn default_index(page: &Page) -> Index {
    let _ = page.doc(); //.map(|d| d);
    None
}
pub fn default_links(page: &Page) -> Links {
    page.doc().map(|d| {
        d.select(Descendant(
            Name("div"),
            Or(Name("p"), Or(Name("table"), Name("ul"))),
        ))
        .map(|a| a.select(Name("a")).into_selection())
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
        .iter()
        .filter_map(|a| a.attr("href"))
        .map(|a| a.to_string())
        .map(Into::into)
        .collect()
    })
}
pub fn default_text(page: &Page) -> Text {
    page.doc().map(|d| {
        ContentType::Text(
            d.select(Child(Name("div"), Name("p")))
                .map(|a| a.parent().unwrap().children().into_selection())
                .max_by(|a, b| a.len().cmp(&b.len()))
                .unwrap()
                .select(Txt)
                .iter()
                .map(|a| a.text())
                .collect(),
            None,
        )
    })
}
pub fn default_images(page: &Page) -> Images {
    page.doc().map(|d| {
        ContentType::Images(
            d.select(Child(Name("div"), Name("img")))
                .map(|a| a.parent().unwrap().select(Name("img")).into_selection())
                .max_by(|a, b| a.len().cmp(&b.len()))
                .unwrap()
                .iter()
                .map(|a| {
                    if let Some(n) = a.attr("src") {
                        n.to_owned()
                    } else {
                        a.attr("data-src")
                            .expect("couldn't find image data-src")
                            .to_owned()
                    }
                })
                .collect(),
            Some(page.origin()),
        )
    })
}
pub fn rr_text(page: &Page) -> Text {
    page.doc().map(|d| {
        // debug!(
        //     "{:?}",
        //     d.select(Or(Descendant(Any, Name("p")), Descendant(Any, Name("br"))))
        //         .map(|a| a.parent().unwrap().children().into_selection())
        //         .max_by(|a, b| {
        //             debug!("len a: {:?}, len b: {:?}", a.len(), b.len());
        //             a.len().cmp(&b.len())
        //         })
        //         .unwrap()
        //         .select(Txt)
        //         .iter()
        //         .map(|a| a.text())
        //         .collect::<Vec<_>>()
        // );
        ContentType::Text(
            d.select(Or(Descendant(Any, Name("p")), Descendant(Any, Name("br"))))
                .map(|a| a.parent().unwrap().children().into_selection())
                .max_by(|a, b| a.len().cmp(&b.len()))
                .unwrap()
                .parent()
                .select(Txt)
                .iter()
                .map(|a| a.text())
                .collect(),
            None,
        )
    })
}
