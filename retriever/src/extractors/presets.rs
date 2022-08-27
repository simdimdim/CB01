use super::{
    content::{
        ImagExtractor,
        LinksExtractor,
        NameExtractor,
        NextExtractor,
        PredExtractor,
        TextExtractor,
    },
    PredType,
    Val,
};
use crate::source::Source;
use select::{
    document::Document,
    predicate::{Child, Descendant, Name, Or, Text},
};
#[derive(Debug, Copy, Clone)]
pub enum Pred {
    Next,
    Custom(&'static str),
}
impl Pred {
    pub fn as_str(&self) -> &str {
        match self {
            Pred::Next => "Next",
            Pred::Custom(s) => s,
        }
    }
}
#[derive(Debug, Clone)]
pub struct FactoryPreset;

impl NameExtractor for FactoryPreset {
    fn name(&self) -> Self::OutputName { "default" }
}
impl PredExtractor for FactoryPreset {
    fn pred(&self) -> PredType { Pred::Next }
}
impl NextExtractor for FactoryPreset {
    fn next(&self, doc: Val<'_>) -> Self::OutputNext {
        Document::from(doc.0)
            .select(Child(Name("a"), Text))
            .filter(|a| a.text().contains(self.pred().as_str()))
            .map(|a| Source::from(a.parent().unwrap().attr("href").unwrap().to_string()))
            .next()
    }
}
impl TextExtractor for FactoryPreset {
    fn text(&self, doc: Val<'_>) -> Self::OutputText {
        Document::from(doc.0)
            .select(Child(Name("div"), Name("p")))
            .map(|a| a.parent().unwrap().children().into_selection())
            .max_by(|a, b| a.len().cmp(&b.len()))
            .unwrap()
            .select(Text)
            .iter()
            .map(|a| a.text())
            .collect()
    }
}
impl ImagExtractor for FactoryPreset {
    fn images(&self, doc: Val<'_>) -> Self::OutputImages {
        Document::from(doc.0)
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
}
impl LinksExtractor for FactoryPreset {
    fn links(&self, doc: Val<'_>) -> Self::OutputLinks {
        Document::from(doc.0)
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
}
