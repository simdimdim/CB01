use crate::{Content, Library, Retriever};
use std::{path::PathBuf, sync::Arc};

#[derive(Debug, Clone, Default)]
pub struct AppData {
    pub library:   Library,
    pub retriever: Arc<Retriever>,
}
