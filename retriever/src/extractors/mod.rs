use crate::source::Source;

use self::{
    content::{
        ImagExtractor,
        LinksExtractor,
        NameExtractor,
        NextExtractor,
        PredExtractor,
        TextExtractor,
    },
    presets::{FactoryPreset, Pred},
    url::{ChapterExtractor, IndexExtractor},
};

use core::fmt::Debug;
use static_init::{dynamic, AccessError};

pub mod content;
pub mod presets;
pub mod url;

pub type NameType = &'static str;
pub type PredType = Pred;
pub type NextType = Option<Source>;
pub type TextType = Vec<String>;
pub type ImagesType = Vec<Source>;
pub type LinksType = Vec<Source>;

/// Associated Types Defaults [#29661](https://github.com/rust-lang/rust/issues/29661)
type Ne = dyn NameExtractor<OutputName = NameType> + Send + Sync;
type Nx = dyn NextExtractor<OutputNext = NextType> + Send + Sync;
type Tx = dyn TextExtractor<OutputText = TextType> + Send + Sync;
type Im = dyn ImagExtractor<OutputImages = ImagesType> + Send + Sync;
type Ln = dyn LinksExtractor<OutputLinks = LinksType> + Send + Sync;

#[dynamic(lazy)]
static mut NAME_EXTRACTORS: Vec<Box<Ne>> = vec![Box::new(FactoryPreset)];
#[dynamic(lazy)]
static mut PREDS: Vec<PredType> = vec![Pred::Next];
#[dynamic(lazy)]
static mut NEXT_EXTRACTORS: Vec<Box<Nx>> = vec![Box::new(FactoryPreset)];
#[dynamic(lazy)]
static mut TEXT_EXTRACTORS: Vec<Box<Tx>> = vec![Box::new(FactoryPreset)];
#[dynamic(lazy)]
static mut IMAG_EXTRACTORS: Vec<Box<Im>> = vec![Box::new(FactoryPreset)];
#[dynamic(lazy)]
static mut LINK_EXTRACTORS: Vec<Box<Ln>> = vec![Box::new(FactoryPreset)];
#[dynamic(lazy)]
pub static mut EXTRACTORS: Vec<Extractor> = vec![Extractor {
    name: 0,
    pred: 0,
    next: 0,
    text: 0,
    imag: 0,
    link: 0,
}];

#[derive(Debug, Clone)]
pub struct Val<'a>(pub &'a str);

#[derive(Debug, Clone)]
pub struct Extractor {
    pub name: usize,
    pub pred: usize,
    pub next: usize,
    pub text: usize,
    pub imag: usize,
    pub link: usize,
}
pub trait Extractors:
    NameExtractor
    + NextExtractor
    + IndexExtractor
    + ChapterExtractor
    + NextExtractor
    + PredExtractor
    + TextExtractor
    + ImagExtractor
    + LinksExtractor {
}

impl Extractor {
    pub fn name(&self) -> Result<<Self as NameExtractor>::OutputName, AccessError> {
        Ok(NAME_EXTRACTORS.try_read()?[self.name].name())
    }

    pub fn pred(&self) -> Result<PredType, AccessError> { Ok(PREDS.try_read()?[self.pred]) }

    pub fn next(&self, doc: Val<'_>) -> Result<<Self as NextExtractor>::OutputNext, AccessError> {
        Ok(NEXT_EXTRACTORS.try_read()?[self.next].next(doc))
    }

    pub fn set_pred(&mut self, idx: Option<usize>) -> Result<bool, AccessError> {
        if let Some(n) = idx {
            if n < PREDS.try_read()?.len() {
                self.pred = n;
                return Ok(true);
            }
        }
        Ok(false)
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
impl TextExtractor for Extractor {
    fn text(&self, doc: Val<'_>) -> Self::OutputText {
        TextExtractor::text(TEXT_EXTRACTORS.read()[self.text].as_ref(), doc)
    }
}
impl ImagExtractor for Extractor {
    fn images(&self, doc: Val<'_>) -> Self::OutputImages {
        ImagExtractor::images(IMAG_EXTRACTORS.read()[self.imag].as_ref(), doc)
    }
}
impl LinksExtractor for Extractor {
    fn links(&self, doc: Val<'_>) -> Self::OutputLinks {
        LinksExtractor::links(LINK_EXTRACTORS.read()[self.link].as_ref(), doc)
    }
}
impl Debug
    for &Box<
        dyn Extractors<
                OutputName = NameType,
                OutputNext = NextType,
                OutputText = TextType,
                OutputImages = ImagesType,
                OutputLinks = LinksType,
            > + Sync
            + 'static,
    >
{
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
pub fn add_new_extractor(from: Option<usize>) -> Result<bool, AccessError> {
    if let Some(r) = EXTRACTORS.try_read()?.get(from.unwrap_or(0)) {
        EXTRACTORS.try_write()?.push(r.clone());
        return Ok(true);
    }
    Ok(false)
}
pub fn add_new_pred(pred: Option<Pred>) -> Result<(), AccessError> {
    match pred {
        Some(pred) => PREDS.try_write()?.push(pred),
        None => PREDS.try_write()?.push(*PREDS.read().last().unwrap()),
    };
    Ok(())
}
pub fn replace_pred(n: usize, pred: Pred) -> Result<bool, AccessError> {
    match PREDS.read().get(n) {
        Some(_) => {
            PREDS.try_write()?.push(pred);
            Ok(true)
        }
        None => Ok(false),
    }
}
