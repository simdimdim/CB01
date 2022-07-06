use crate::{Label, Page};
use reqwest::header::HeaderMap;
use select::{
    document::Document,
    predicate::{Child, Descendant, Name, Or, Text},
};

pub type HTMLStr<'a> = &'a str;
pub trait Finder: std::fmt::Debug + Send + Sync {
    fn name(&self) -> &str;
    fn pred(&self) -> &str { "Next" }
    fn split_by(&self) -> &str { " Chapter" }
    fn num(&self, page: &Page) -> (u16, u16, String) {
        let segments = page
            .url
            .path_segments()
            .unwrap()
            .rev()
            .filter(|&a| a.is_empty())
            .collect::<Vec<_>>();
        let numbers = segments
            .iter()
            .map(|a| {
                a.matches(char::is_numeric)
                    .collect::<Vec<&str>>()
                    .join("")
                    .parse::<u16>()
                    .unwrap_or_default()
            })
            .collect::<Vec<u16>>();
        // TODO: do a better job at finding the index
        let index_candidate = if segments.len() < 3 {
            segments.iter().last()
        } else {
            segments.iter().rev().nth(1)
        };
        match (numbers.as_slice(), index_candidate) {
            ([x @ 0..=9000, y @ 0..=9000, ..], Some(&z)) => {
                (*x, *y, z.to_string())
            }
            ([x @ 0..=9000], Some(z)) => (0, *x, z.to_string()),
            ([], Some(z)) => (0, 0, z.to_string()),
            _ => (0, 0, "".to_string()),
        }
    }
    fn title(&self, doc: HTMLStr<'_>) -> Label {
        let title = Document::from(doc)
            .select(Name("title"))
            .into_selection()
            .first()
            .unwrap()
            .text();
        if title.contains(self.split_by()) {
            title
                .split(self.split_by())
                .filter(|&a| !a.is_empty())
                .collect::<Vec<_>>()
                .first()
                .unwrap()
                .to_string()
        } else {
            title
        }
        .into()
    }
    fn index(&self, _doc: HTMLStr<'_>) -> Option<Page> { None }
    fn links(&self, doc: HTMLStr<'_>) -> Vec<Page> {
        Document::from(doc)
            .select(Descendant(
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
        /* TODO: Add a similarity check and only return the biggest cluster of
        similar links */
    }
    fn next(&self, doc: HTMLStr<'_>) -> Option<Page> {
        Document::from(doc)
            .select(Child(Name("a"), Text))
            .filter(|a| a.text().contains(self.pred()))
            .map(|a| {
                Page::from(a.parent().unwrap().attr("href").unwrap().to_string())
            })
            .next()
        /* TODO: Add a similarity check and only return the biggest cluster of
        similar links */
    }
    fn text(&self, doc: HTMLStr<'_>) -> Vec<String> {
        Document::from(doc)
            .select(Child(Name("div"), Name("p")))
            .map(|a| a.parent().unwrap().children().into_selection())
            .max_by(|a, b| a.len().cmp(&b.len()))
            .unwrap()
            .select(Text)
            .iter()
            .map(|a| a.text())
            .collect()
    }
    fn images(&self, doc: HTMLStr<'_>) -> Vec<Page> {
        Document::from(doc)
            .select(Child(Name("div"), Name("img")))
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
            .map(Into::into)
            .collect()
        /* TODO: Similar to index() add a check for links similarity */
    }
    fn headers(&self) -> HeaderMap { HeaderMap::new() }
}

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DefaultFinder;
impl Finder for DefaultFinder {
    fn name(&self) -> &str { "default" }
}
