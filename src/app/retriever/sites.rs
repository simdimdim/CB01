use crate::Finder;
use reqwest::header::{HeaderMap, REFERER};
use std::collections::HashMap;
use url::Host;

pub(crate) struct Include;
impl Include {
    pub fn custom(hm: &mut HashMap<Host, Box<dyn Finder>>) {
        hm.insert(
            Host::parse("https://manganato.com/").unwrap(),
            Box::new(Manganato),
        );
    }
}

#[derive(Debug)]
struct Manganato;
impl Finder for Manganato {
    fn pred(&self) -> &str { "NEXT" }

    fn headers(&self) -> HeaderMap {
        let mut hm = HeaderMap::new();
        hm.insert(REFERER, "https://manganato.com/".parse().unwrap());
        hm
    }
}
