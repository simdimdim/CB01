use crate::{Label, Page};
use select::{
    document::Document,
    predicate::{Child, Descendant, Name, Or, Text},
};

type Input<'a> = &'a Option<Document>;

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DefaultFinder;

pub trait Finder:
    TextFinder + TitleFinder + LinksFinder + NextFinder + ImageFinder {
    fn title(&self, doc: Input) -> Label {
        self.title_def(doc.as_ref().expect("HTML not found."))
    }
    fn links(&self, doc: Input) -> Vec<Page> {
        self.links_def(doc.as_ref().expect("HTML not found."))
    }
    fn next(&self, doc: Input) -> Option<Page> {
        self.next_def(doc.as_ref().expect("HTML not found."))
    }
    fn text(&self, doc: Input) -> Vec<String> {
        self.text_def(doc.as_ref().expect("HTML not found."))
    }
    fn images(&self, doc: Input) -> Vec<Page> {
        self.images_def(doc.as_ref().expect("HTML not found."))
    }
}

pub trait TitleFinder {
    fn title_def(&self, doc: &Document) -> Label {
        let title = doc
            .select(Name("title"))
            .into_selection()
            .first()
            .unwrap()
            .text();
        if title.contains(" Chapter") {
            title
                .split(" Chapter")
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
}
pub trait LinksFinder {
    fn links_def(&self, doc: &Document) -> Vec<Page> {
        doc.select(Descendant(
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
}
pub trait NextFinder {
    fn next_def(&self, doc: &Document) -> Option<Page> {
        doc.select(Child(Name("a"), Text))
            .filter(|a| a.text().contains("Next"))
            .map(|a| {
                Page::from(a.parent().unwrap().attr("href").unwrap().to_string())
            })
            .next()
        /* TODO: Add a similarity check and only return the biggest cluster of
        similar links */
    }
}
pub trait TextFinder {
    fn text_def(&self, doc: &Document) -> Vec<String> {
        doc.select(Child(Name("div"), Name("p")))
            .map(|a| a.parent().unwrap().children().into_selection())
            .max_by(|a, b| a.len().cmp(&b.len()))
            .unwrap()
            .select(Text)
            .iter()
            .map(|a| a.text())
            .collect()
    }
}
pub trait ImageFinder {
    fn images_def(&self, doc: &Document) -> Vec<Page> {
        doc.select(Child(Name("div"), Name("img")))
            .map(|a| a.parent().unwrap().select(Name("img")).into_selection())
            .max_by(|a, b| a.len().cmp(&b.len()))
            .unwrap()
            .iter()
            .map(|a| a.attr("src").unwrap().to_string())
            .map(Into::into)
            .collect()
        /* TODO: Similar to index() add a check for links similarity */
    }
}
impl Finder for DefaultFinder {}
impl TitleFinder for DefaultFinder {}
impl LinksFinder for DefaultFinder {}
impl NextFinder for DefaultFinder {}
impl TextFinder for DefaultFinder {}
impl ImageFinder for DefaultFinder {}
