use crate::{page::Page, presets::*, Images, Index, Links, Next, Text, Title};
use std::fmt::Debug;

#[derive(Clone)]
pub struct Extractor {
    title: Option<fn(&Page) -> Title>,
    index: Option<fn(&Page) -> Index>,
    next: Option<fn(&Page) -> Next>,
    links: Option<fn(&Page) -> Links>,
    text: Option<fn(&Page) -> Text>,
    images: Option<fn(&Page) -> Images>,
}
#[derive(Debug, Clone, Default)]
pub struct Manifest {
    name: String,
    title: usize,
    next_by: usize,
    split_by: usize,
    next: usize,
    text: usize,
    images: usize,
    links: usize,
    index: usize,
}

impl Manifest {
    pub fn rename<T: Into<String>>(&mut self, name: T) -> &mut Self {
        self.name = name.into();
        self
    }

    pub fn name(&self) -> &String { &self.name }

    pub fn title(&self) -> usize { self.title }

    pub fn next_by(&self) -> usize { self.next_by }

    pub fn split_by(&self) -> usize { self.split_by }

    pub fn next(&self) -> usize { self.next }

    pub fn text(&self) -> usize { self.text }

    pub fn images(&self) -> usize { self.images }

    pub fn links(&self) -> usize { self.links }

    pub fn index(&self) -> usize { self.index }
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

    pub async fn get_title(&self, page: &Page) -> Title { self.title.and_then(|f| f(page)) }

    pub async fn get_next(&self, page: &Page) -> Next { self.next.and_then(|f| f(page)) }

    pub async fn get_index(&self, page: &Page) -> Index { self.index.and_then(|f| f(page)) }

    pub async fn get_links(&self, page: &Page) -> Links { self.links.and_then(|f| f(page)) }

    pub async fn get_text(&self, page: &Page) -> Text { self.text.and_then(|f| f(page)) }

    pub async fn get_images(&self, page: &Page) -> Images { self.images.and_then(|f| f(page)) }

    pub fn set_title(&mut self, f: Option<fn(&Page) -> Title>) { self.title = f; }

    pub fn set_next(&mut self, f: Option<fn(&Page) -> Index>) { self.next = f; }

    pub fn set_index(&mut self, f: Option<fn(&Page) -> Next>) { self.index = f; }

    pub fn set_links(&mut self, f: Option<fn(&Page) -> Links>) { self.links = f; }

    pub fn set_text(&mut self, f: Option<fn(&Page) -> Text>) { self.text = f; }

    pub fn set_images(&mut self, f: Option<fn(&Page) -> Images>) { self.images = f; }
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
