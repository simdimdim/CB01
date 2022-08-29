use self::presets::{
    default_images,
    default_index,
    default_links,
    default_next,
    default_text,
    default_title,
    Method,
};
use crate::source::{Page, Source};
use core::fmt::Debug;
use dashmap::DashMap;

pub mod presets;

#[macro_export]
macro_rules! prepare {
    ($func_name:ident) => {
        Method::from($func_name as fn(_) -> _)
    };
}

pub type TitleType = String;
pub type NextType = Option<Source>;
pub type TextType = Vec<String>;
pub type ImagesType = Vec<Source>;
pub type LinksType = Vec<Source>;

#[derive(Debug, Clone)]
pub struct HoundDesc {
    name: String,
    title: usize,
    split: usize,
    pred: usize,
    next: usize,
    text: usize,
    imag: usize,
    link: usize,
    index: usize,
}

impl HoundDesc {
    pub fn rename(&mut self, name: String) { self.name = name; }

    pub fn name(&self) -> &String { &self.name }

    pub fn title(&self) -> usize { self.title }

    pub fn split(&self) -> usize { self.split }

    pub fn pred(&self) -> usize { self.pred }

    pub fn next(&self) -> usize { self.next }

    pub fn text(&self) -> usize { self.text }

    pub fn imag(&self) -> usize { self.imag }

    pub fn link(&self) -> usize { self.link }

    pub fn index(&self) -> usize { self.index }
}

#[derive(Debug, Clone, Default)]
pub struct DogHouse {
    #[allow(dead_code)]
    targets: DashMap<String, Page>,
    hounds: Vec<HoundDesc>,
    pred: Vec<String>,
    split: Vec<String>,
    title: Vec<Method<(String, String), TitleType>>,
    next: Vec<Method<(String, String), NextType>>,
    text: Vec<Method<String, TextType>>,
    images: Vec<Method<String, ImagesType>>,
    links: Vec<Method<String, LinksType>>,
    index: Vec<Method<String, NextType>>,
}
impl DogHouse {
    pub fn new() -> Self {
        let targets = DashMap::new();
        let hounds = vec![HoundDesc {
            name: "Default".to_owned(),
            title: 0,
            split: 0,
            pred: 0,
            next: 0,
            text: 0,
            imag: 0,
            link: 0,
            index: 0,
        }];
        Self {
            targets,
            hounds,
            pred: vec!["Next", "NEXT", "->"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            split: vec![" Chapter"].iter().map(|s| s.to_string()).collect(),
            title: vec![prepare!(default_title)],
            next: vec![prepare!(default_next)],
            text: vec![prepare!(default_text)],
            images: vec![prepare!(default_images)],
            links: vec![prepare!(default_links)],
            index: vec![prepare!(default_index)],
        }
    }

    //TODO: extract all data from a Page at the same time

    pub fn title(&self, doc: String, by: usize, split: Option<usize>) -> TitleType {
        let hound = self.hounds.get(by).unwrap().title;
        let method = self.title.get(hound).unwrap_or_else(|| &self.title[0]);
        let split = self.split.get(split.unwrap_or(0)).unwrap().into();
        method.apply((doc, split))
    }

    pub fn next(&self, doc: String, by: usize, pred: Option<usize>) -> NextType {
        let hound = self.hounds.get(by).unwrap().title;
        let method = self.next.get(hound).unwrap_or_else(|| &self.next[0]);
        let pred = self.pred.get(pred.unwrap_or(0)).unwrap().into();
        method.apply((doc, pred))
    }

    pub fn text(&self, doc: String, by: usize) -> TextType {
        let hound = self.hounds.get(by).unwrap().title;
        let method = self.text.get(hound).unwrap_or_else(|| &self.text[0]);
        method.apply(doc)
    }

    pub fn images(&self, doc: String, by: usize) -> ImagesType {
        let hound = self.hounds.get(by).unwrap().title;
        let method = self.images.get(hound).unwrap_or_else(|| &self.images[0]);
        method.apply(doc)
    }

    pub fn links(&self, doc: String, by: usize) -> LinksType {
        let hound = self.hounds.get(by).unwrap().title;
        let method = self.links.get(hound).unwrap_or_else(|| &self.links[0]);
        method.apply(doc)
    }

    pub fn index(&self, doc: String, by: usize) -> NextType {
        let hound = self.hounds.get(by).unwrap().title;
        let method = self.index.get(hound).unwrap_or_else(|| &self.index[0]);
        method.apply(doc)
    }

    pub fn add_new_extractor(&mut self, from: Option<usize>) -> bool {
        if let Some(r) = self.hounds.get(from.unwrap_or(0)) {
            self.hounds.insert(self.hounds.len(), r.clone());
            return true;
        }
        false
    }

    pub fn add_new_pred(&mut self, pred: Option<String>) {
        match pred {
            Some(pred) => self.pred.push(pred),
            None => self.pred.push(self.pred.last().unwrap().into()),
        }
    }

    pub fn replace_pred(&mut self, n: usize, pred: String) -> bool {
        match self.pred.get(n) {
            Some(_) => {
                self.pred.push(pred);
                true
            }
            None => false,
        }
    }
}
