use crate::{Finder, HTMLStr, Page};
use reqwest::header::{HeaderMap, REFERER};
use select::{
    document::Document,
    predicate::{Child, Class, Name},
};
use std::collections::HashMap;
use url::Host;

pub(crate) struct Include;
impl Include {
    pub fn custom(
        find: &mut Vec<Box<dyn Finder>>, hm: &mut HashMap<Host, usize>,
    ) {
        hm.insert(Host::parse("manganato.com").unwrap(), find.len());
        hm.insert(Host::parse("readmanganato.com").unwrap(), find.len());
        find.push(Box::new(ManganatoCom));
        hm.insert(Host::parse("zinmanga.com").unwrap(), find.len());
        find.push(Box::new(ZimangaCom));
    }
}

#[derive(Debug)]
struct ManganatoCom;
impl Finder for ManganatoCom {
    fn name(&self) -> &str { "readmanganato" }

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
    fn name(&self) -> &str { "zinmanga" }

    fn pred(&self) -> &str { "Next" }

    fn headers(&self) -> HeaderMap {
        let mut hm = HeaderMap::new();
        hm.insert(REFERER, "https://zinmanga.com/".parse().unwrap());
        hm
    }

    // fn index(&self, doc: HTMLStr<'_>) -> Option<Page> {
    //     let res = Document::from(doc)
    //         .select(Child(Class("breadcrumb"), Name("a")))
    //         .map(|a| a.attr("href").unwrap().to_string().into())
    //         .nth(2);
    //     res
    // }

    fn images(&self, doc: HTMLStr<'_>) -> Vec<Page> {
        let res = Document::from(doc)
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
