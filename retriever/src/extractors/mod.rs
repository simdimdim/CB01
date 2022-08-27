use self::{
    // content::{ImagesExtractor, LinksExtractor, TexttExtractor},
    defaulte::DefaultNext,
    name::{NameExtractor, NameType},
    pred::{NextExtractor, NextType, Pred, PredExtractor, PredType},
    url::{ChapterExtractor, IndexExtractor},
};

use core::fmt::Debug;
use static_init::dynamic;

pub mod content;
pub mod defaulte;
pub mod name;
pub mod pred;
pub mod url;

type Nex = dyn NameExtractor<OutputName = NameType> + Send + Sync;
type Prd = PredType;
type Nxt = dyn NextExtractor<OutputNext = NextType> + Send + Sync;

#[dynamic(lazy)]
static mut NAME_EXTRACTORS: Vec<Box<Nex>> = vec![Box::new(Val("default"))];
#[dynamic(lazy)]
static mut PREDS: Vec<Prd> = vec![Pred::Next];
#[dynamic(lazy)]
static mut NEXT_EXTRACTORS: Vec<Box<Nxt>> = vec![Box::new(DefaultNext)];
#[dynamic(lazy)]
pub static mut EXTRACTORS: Vec<Extractor> = vec![Extractor {
    name: 0,
    pred: 0,
    next: 0,
}];

#[derive(Debug, Clone)]
pub struct Val<'a>(pub &'a str);

#[derive(Debug, Clone)]
pub struct Extractor {
    pub name: usize,
    pub pred: usize,
    pub next: usize,
}
pub trait Extractors:
    NameExtractor + NextExtractor + ChapterExtractor + IndexExtractor + NextExtractor + PredExtractor
// + ImagesExtractor
// + LinksExtractor
// + TexttExtractor {
{
}

impl Extractor {
    pub fn name(&self) -> <Self as NameExtractor>::OutputName {
        NAME_EXTRACTORS.read()[self.name].name()
    }

    pub fn pred(&self) -> PredType { PREDS.read()[self.pred] }

    pub fn next(&self, doc: Val<'_>) -> <Self as NextExtractor>::OutputNext {
        NEXT_EXTRACTORS.read()[self.next].next(doc)
    }

    pub fn set_pred(&mut self, idx: Option<usize>) -> bool {
        if let Some(n) = idx {
            if n < PREDS.read().len() {
                self.pred = n;
                return true;
            }
        }
        false
    }
}
impl NameExtractor for Extractor {
    fn name(&self) -> Self::OutputName {
        NameExtractor::name(NAME_EXTRACTORS.read()[self.next].as_ref())
    }
}
impl PredExtractor for Extractor {
    fn pred(&self) -> PredType { PREDS.read()[self.pred] }
}
impl NextExtractor for Extractor {
    fn next(&self, doc: Val<'_>) -> Self::OutputNext {
        NextExtractor::next(NEXT_EXTRACTORS.read()[self.next].as_ref(), doc)
    }
}

impl Debug for &Box<dyn Extractors<OutputName = NameType, OutputNext = NextType> + Sync + 'static> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Extractors{{name:{},\npred:{:?},\n{:?}}}",
            self.name(),
            self.pred(),
            self.split_by(),
        )
    }
}
pub fn add_new_extractor(from: Option<usize>) -> bool {
    if let Some(r) = EXTRACTORS.read().get(from.unwrap_or(0)) {
        EXTRACTORS.write().push(r.clone());
        return true;
    }
    false
}
pub fn add_new_pred(pred: Option<Pred>) {
    match pred {
        Some(pred) => PREDS.write().push(pred),
        None => PREDS.write().push(*PREDS.read().last().unwrap()),
    }
}
pub fn replace_pred(n: usize, pred: Pred) -> bool {
    match PREDS.read().get(n) {
        Some(_) => {
            PREDS.write().push(pred);
            true
        }
        None => false,
    }
}
