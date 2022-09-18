use crate::{extractor::Manifest, page::Page};
use core::fmt::Debug;
use dashmap::DashMap;
use reqwest::{header::HeaderMap, Client};
use url::Host;

#[macro_export]
macro_rules! prepare {
    ($func_name:ident) => {
        Method::from($func_name as fn(_) -> _)
    };
}

pub type TitleType = String;
pub type NextType = Option<Page>;
pub type TextType = Vec<String>;
pub type ImagesType = Vec<Page>;
pub type LinksType = Vec<Page>;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Retriever {
    referers: DashMap<Host, HeaderMap>,
    targets: DashMap<Host, Page>,
    manifests: Vec<Manifest>,
    client: Client,
}

impl Retriever {
    pub fn new() -> Self {
        #[allow(clippy::needless_update)]
        Self {
            referers: DashMap::new(),
            targets: DashMap::new(),
            manifests: vec![],
            client: Client::default(),
            ..Default::default()
        }
    }

    //TODO: extract all data from a Page at the same time

    pub fn add_extractor(&mut self, from: Option<usize>) -> bool {
        if let Some(r) = self.manifests.get(from.unwrap_or(0)) {
            self.manifests.insert(self.manifests.len(), r.clone());
            return true;
        }
        false
    }

    /// false if text, true if images
    pub async fn guess_type(&self, _src: &Page) -> bool { false }
}

impl Default for Retriever {
    fn default() -> Self {
        let referers = DashMap::new();
        let targets = DashMap::new();
        let manifests = vec![{
            let mut h = Manifest::default();
            h.rename("Default");
            h
        }];
        let client = Client::default();
        Self {
            referers,
            targets,
            manifests,
            client,
        }
    }
}
