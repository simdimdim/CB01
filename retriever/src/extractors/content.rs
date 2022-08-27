use super::{ImagesType, LinksType, NameType, NextType, PredType, TextType, Val};
use std::fmt::Debug;

pub trait NameExtractor {
    type OutputName = NameType;
    fn name(&self) -> Self::OutputName;
}
pub trait PredExtractor {
    fn pred(&self) -> PredType;
}
pub trait NextExtractor: PredExtractor {
    type OutputNext = NextType;
    fn next(&self, doc: Val<'_>) -> Self::OutputNext;
}
pub trait TextExtractor {
    type OutputText = TextType;
    fn text(&self, doc: Val<'_>) -> Self::OutputText;
}
pub trait ImagExtractor {
    type OutputImages = ImagesType;
    fn images(&self, doc: Val<'_>) -> Self::OutputImages;
}
pub trait LinksExtractor {
    type OutputLinks = LinksType;
    fn links(&self, doc: Val<'_>) -> Self::OutputLinks;
}
impl<T: Debug> Debug for dyn NameExtractor<OutputName = T> + Send + Sync + 'static {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Name:{{name:{:?}}}", self.name()))
    }
}
impl<T: Debug> Debug for dyn NextExtractor<OutputNext = T> + Send + Sync + 'static {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Next:{{pred:{:?}}}", self.pred()))
    }
}
impl<T: Debug> Debug for dyn TextExtractor<OutputText = T> + Send + Sync + 'static {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Text:{{{:?}}}", self))
    }
}
impl<T: Debug> Debug for dyn ImagExtractor<OutputImages = T> + Send + Sync + 'static {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Images:{{{:?}}}", self))
    }
}
impl<T: Debug> Debug for dyn LinksExtractor<OutputLinks = T> + Send + Sync + 'static {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Links:{{{:?}}}", self))
    }
}
