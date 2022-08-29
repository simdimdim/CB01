use super::{ImagesType, LinksType, NextType, TextType, TitleType};
use crate::source::Source;
use select::{
    document::Document,
    predicate::{Child, Descendant, Name, Or, Text},
};

#[derive(Debug, Clone)]
pub struct Method<I, O> {
    name: String,
    operator: fn(I) -> O,
}
impl<I, O> Method<I, O> {
    pub fn name(&self) -> &String { &self.name }

    pub fn rename(&mut self, name: String) { self.name = name; }

    pub fn apply(&self, input: I) -> O { (self.operator)(input) }
}
impl<I, O> From<fn(I) -> O> for Method<I, O> {
    fn from(operator: fn(I) -> O) -> Self {
        use std::any::type_name;
        fn type_name_of<T>(_: T) -> String { type_name::<T>().to_string() }
        Method {
            name: type_name_of(operator),
            operator,
        }
    }
}
pub(crate) fn default_title((doc, split): (String, String)) -> TitleType {
    let title = Document::from(doc.as_str())
        .select(Name("title"))
        .into_selection()
        .first()
        .unwrap()
        .text();
    #[allow(clippy::useless_conversion)]
    if title.contains(split.as_str()) {
        title
            .split(split.as_str())
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
pub(crate) fn default_next((doc, pred): (String, String)) -> NextType {
    Document::from(doc.as_str())
        .select(Child(Name("a"), Text))
        .filter(|a| a.text().contains(pred.as_str()))
        .map(|a| Source::from(a.parent().unwrap().attr("href").unwrap().to_string()))
        .next()
}
pub(crate) fn default_text(doc: String) -> TextType {
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
pub(crate) fn default_images(doc: String) -> ImagesType {
    Document::from(doc.as_str())
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
pub(crate) fn default_links(doc: String) -> LinksType {
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
pub(crate) fn default_index(_doc: String) -> Option<Source> { None }
