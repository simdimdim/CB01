use crate::{Library, Retriever};
use std::sync::Arc;

#[derive(Debug, Clone, Default)]
pub struct AppData {
    pub library:   Library,
    pub retriever: Arc<Retriever>,
}
