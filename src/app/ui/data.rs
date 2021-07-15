use crate::{Content, Library, Retriever};
use std::{path::PathBuf, sync::Arc};

#[derive(Debug, Clone, Default)]
pub struct AppData {
    pub library:   Library,
    pub retriever: Arc<Retriever>,
    pub current:   Box<Vec<Content>>,
    pub flipped:   bool,
    pub reversed:  bool,
}
