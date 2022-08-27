use crate::source::Source;
use std::fmt::Debug;

use super::Val;

pub type PredType = Pred;
pub type NextType = Option<Source>;
pub trait PredExtractor {
    fn pred(&self) -> PredType;
}
pub trait NextExtractor: PredExtractor {
    type OutputNext = NextType;
    fn next(&self, doc: Val<'_>) -> Self::OutputNext;
}

impl<T: Debug> Debug for dyn NextExtractor<OutputNext = T> + Send + Sync + 'static {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Pred:{{pred:{:?}}}", self.pred()))
    }
}
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
