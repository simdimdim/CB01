use crate::source::Source;
use std::fmt::Debug;

pub trait TexttExtractor {
    type OutputText = Vec<String>;
    fn text(&self) -> Self::OutputText;
}
pub trait ImagesExtractor {
    type OutputImages = Vec<Source>;
    fn images(&self) -> Self::OutputImages;
}
pub trait LinksExtractor {
    type OutputLinks = Vec<Source>;
    fn links(&self) -> Self::OutputLinks;
}

impl<T: Debug> Debug for dyn TexttExtractor<OutputText = T> + Send + Sync + 'static {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Name:{{name:{:?}}}", self.text()))
    }
}
impl<T: Debug> Debug for dyn ImagesExtractor<OutputImages = T> + Send + Sync + 'static {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Name:{{name:{:?}}}", self.images()))
    }
}
impl<T: Debug> Debug for dyn LinksExtractor<OutputLinks = T> + Send + Sync + 'static {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Name:{{name:{:?}}}", self.links()))
    }
}
