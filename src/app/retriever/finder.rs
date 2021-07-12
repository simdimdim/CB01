use crate::{Label, Page};
use reqwest::header::HeaderMap;
use select::{
    document::Document,
    predicate::{Child, Descendant, Name, Or, Text},
};

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DefaultFinder;

type Input<'a> = &'a String;

pub trait Finder: std::fmt::Debug + Send + Sync {
    fn pred(&self) -> &str { "Next" }
    fn split_by(&self) -> &str { " Chapter" }
    fn num(&self, page: &Page) -> (u16, u16, String) {
        let segments = page
            .url
            .path_segments()
            .unwrap()
            .rev()
            .filter(|&a| a != "")
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
            segments.iter().rev().skip(1).next()
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
    fn title(&self, doc: Input) -> Label {
        let title = Document::from(doc.as_str())
            .select(Name("title"))
            .into_selection()
            .first()
            .unwrap()
            .text();
        if title.contains(self.split_by()) {
            title
                .split(self.split_by())
                .filter(|&a| a != "")
                .collect::<Vec<_>>()
                .first()
                .unwrap()
                .to_string()
        } else {
            title
        }
        .into()
    }
    fn index(&self, _doc: Input) -> Option<Page> { None }
    fn links(&self, doc: Input) -> Vec<Page> {
        Document::from(doc.as_str())
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
    fn next(&self, doc: Input) -> Option<Page> {
        Document::from(doc.as_str())
            .select(Child(Name("a"), Text))
            .filter(|a| a.text().contains(self.pred()))
            .map(|a| {
                Page::from(a.parent().unwrap().attr("href").unwrap().to_string())
            })
            .next()
        /* TODO: Add a similarity check and only return the biggest cluster of
        similar links */
    }
    fn text(&self, doc: Input) -> Vec<String> {
        Document::from(doc.as_str())
            .select(Child(Name("div"), Name("p")))
            .map(|a| a.parent().unwrap().children().into_selection())
            .max_by(|a, b| a.len().cmp(&b.len()))
            .unwrap()
            .select(Text)
            .iter()
            .map(|a| a.text())
            .collect()
    }
    fn images(&self, doc: Input) -> Vec<Page> {
        Document::from(doc.as_str())
            .select(Child(Name("div"), Name("img")))
            .map(|a| a.parent().unwrap().select(Name("img")).into_selection())
            .max_by(|a, b| a.len().cmp(&b.len()))
            .unwrap()
            .iter()
            .map(|a| a.attr("src").unwrap().to_string())
            .map(Into::into)
            .collect()
        /* TODO: Similar to index() add a check for links similarity */
    }
    fn headers(&self) -> HeaderMap { HeaderMap::new() }
}

impl Finder for DefaultFinder {}
