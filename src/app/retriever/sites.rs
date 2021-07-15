use crate::{Finder, Page};
use reqwest::header::{HeaderMap, REFERER};
use select::{
    document::Document,
    predicate::{Child, Name},
};
use std::collections::HashMap;
use url::Host;

type Input<'a> = &'a String;

pub(crate) struct Include;
impl Include {
    pub fn custom(hm: &mut HashMap<Host, Box<dyn Finder>>) {
        hm.insert(
            Host::parse("manganato.com").unwrap(),
            Box::new(ManganatoCom),
        );
        hm.insert(
            Host::parse("readmanganato.com").unwrap(),
            Box::new(ManganatoCom),
        );
        hm.insert(
            Host::parse("s18.mkklcdnv6tempv4.com").unwrap(),
            Box::new(ManganatoCom),
        );
        hm.insert(
            Host::parse("bu4.mkklcdnv6tempv4.com").unwrap(),
            Box::new(ManganatoCom),
        );
        hm.insert(Host::parse("zinmanga.com").unwrap(), Box::new(ZimangaCom));
        hm.insert(
            Host::parse("z-cdn.zinmanga.com").unwrap(),
            Box::new(ZimangaCom),
        );
    }
}

#[derive(Debug)]
struct ManganatoCom;
impl Finder for ManganatoCom {
    fn pred(&self) -> &str { "NEXT" }

    fn headers(&self) -> HeaderMap {
        let mut hm = HeaderMap::new();
        hm.insert(REFERER, "https://readmanganato.com/".parse().unwrap());
        hm
    }
}

#[derive(Debug)]
struct ZimangaCom;
impl Finder for ZimangaCom {
    fn pred(&self) -> &str { "Next" }

    fn headers(&self) -> HeaderMap {
        let mut hm = HeaderMap::new();
        hm.insert(REFERER, "https://zinmanga.com/".parse().unwrap());
        hm
    }

    fn images(&self, doc: Input) -> Vec<Page> {
        let res = Document::from(doc.as_str())
            .select(Child(Child(Name("div"), Name("div")), Name("img")))
            .map(|a| {
                a.parent()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .select(Name("div"))
                    .into_selection()
            })
            .max_by(|a, b| a.len().cmp(&b.len()))
            .map(|a| a.select(Name("img")))
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
            .map(Into::into)
            .collect();
        /* TODO: Similar to index() add a check for links similarity */
        res
    }
}
