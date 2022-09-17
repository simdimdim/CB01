use crate::page::Page;
use core::fmt::Debug;
use dashmap::DashMap;
use reqwest::{header::HeaderMap, Client};
use url::Host;

pub mod presets;

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

#[derive(Debug, Clone, Default)]
pub struct HoundPack {
    name: String,
    title: usize,
    pred: usize,
    split: usize,
    next: usize,
    text: usize,
    images: usize,
    links: usize,
    index: usize,
}

impl HoundPack {
    pub fn rename<T: Into<String>>(&mut self, name: T) { self.name = name.into(); }

    pub fn name(&self) -> &String { &self.name }

    pub fn title(&self) -> usize { self.title }

    pub fn pred(&self) -> usize { self.pred }

    pub fn split(&self) -> usize { self.split }

    pub fn next(&self) -> usize { self.next }

    pub fn text(&self) -> usize { self.text }

    pub fn images(&self) -> usize { self.images }

    pub fn links(&self) -> usize { self.links }

    pub fn index(&self) -> usize { self.index }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Retriever {
    referers: DashMap<Host, HeaderMap>,
    targets: DashMap<Host, Page>,
    reigster: Vec<HoundPack>,
    client: Client,
}

impl Retriever {
    pub fn new() -> Self {
        #[allow(clippy::needless_update)]
        Self {
            referers: DashMap::new(),
            targets: DashMap::new(),
            reigster: vec![],
            client: Client::default(),
            ..Default::default()
        }
    }

    //TODO: extract all data from a Page at the same time

    pub fn add_extractor(&mut self, from: Option<usize>) -> bool {
        if let Some(r) = self.reigster.get(from.unwrap_or(0)) {
            self.reigster.insert(self.reigster.len(), r.clone());
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
        let hounds = vec![{
            let mut h = HoundPack::default();
            h.rename("Default");
            h
        }];
        let client = Client::default();
        Self {
            referers,
            targets,
            reigster: hounds,
            client,
        }
    }
}
