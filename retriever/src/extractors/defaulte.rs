use crate::source::Source;

use super::{
    name::NameExtractor,
    pred::{NextExtractor, Pred, PredExtractor, PredType},
    Val,
};
use select::{
    document::Document,
    predicate::{Child, Name as TName, Text},
};

#[derive(Debug, Clone)]
pub struct DefaultNext;

impl NameExtractor for Val<'static> {
    fn name(&self) -> Self::OutputName { self.0 }
}
impl PredExtractor for DefaultNext {
    fn pred(&self) -> PredType { Pred::Next }
}
impl NextExtractor for DefaultNext {
    fn next(&self, doc: Val<'_>) -> Self::OutputNext {
        Document::from(doc.0)
            .select(Child(TName("a"), Text))
            .filter(|a| a.text().contains(self.pred().as_str()))
            .map(|a| Source::from(a.parent().unwrap().attr("href").unwrap().to_string()))
            .next()
    }
}
