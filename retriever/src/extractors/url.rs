use std::fmt::Debug;

pub trait ChapterExtractor {
    fn split_by(&self) -> &str;
}
pub trait IndexExtractor {
    fn index(&self) -> Option<String> { None }
}

impl Debug for dyn ChapterExtractor + Send + Sync + 'static {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Chapter:{{split_by:{:?}}}", self.split_by()))
    }
}
impl Debug for dyn IndexExtractor + Send + Sync + 'static {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Index:{{{:?}}}", self.index()))
    }
}
