use crate::{AppSettings, Book, File, Id, Library, Retriever, APP_NAME};
use std::{
    collections::{BTreeMap, HashMap},
    fs::OpenOptions,
    sync::Arc,
};

#[derive(Debug, Clone, Default)]
pub struct AppData {
    pub library:   Library,
    pub retriever: Arc<Retriever>,
}
impl AppData {
    pub fn load_library(&mut self) {
        // TODO: Load library from disc.
        let settings: AppSettings =
            confy::load(APP_NAME).expect("Failed to load settings.");
        let books_json = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&settings.db_location.join("library.json"))
            .expect("Failed to open books.json.");
        // let groups: HashMap<String, HashSet<Id>>;
        let mut library = Library::default();
        let books = serde_json::from_reader(books_json).unwrap_or_default();
        library.books = books;
        let books_json = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&settings.db_location.join("books.json"))
            .expect("Failed to open books.json.");
        let books = serde_json::from_reader(books_json).unwrap_or_default();
        library.books = books;
        self.library = library;
    }
}
