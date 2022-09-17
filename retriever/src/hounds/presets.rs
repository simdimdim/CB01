use crate::{
    page::{ContentType, Page},
    Images,
    Index,
    Links,
    Next,
    Text,
    Title,
};
use futures::future::join_all;
use log::info;
use select::predicate::{Any, Child, Descendant, Name, Or, Text as Txt};
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
