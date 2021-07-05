use std::{
    collections::{BTreeMap, HashMap},
    sync::atomic::{AtomicU16, Ordering},
};

pub mod book;
pub mod chapter;
pub mod content;
pub use book::*;
pub use chapter::*;
pub use content::*;

pub(self) static ID_COUNTER: AtomicU16 = AtomicU16::new(0);

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Label(String);
#[derive(Debug, Clone, Default)]
pub struct Library {
    pub titles: HashMap<Label, u16>,
    pub books:  BTreeMap<u16, Book>,
    pub num:    Id,
}

impl Library {
    pub fn book(&mut self, name: &Label) -> &Book {
        let id = self.book_id(name).clone();
        match self.books.contains_key(&id) {
            true => self.books.get(&id).unwrap(),
            false => {
                self.books.insert(id, Book {
                    id,
                    ..Default::default()
                });
                self.books.get(&id).unwrap()
            }
        }
    }

    pub fn book_mut(&mut self, name: &Label) -> &mut Book {
        let id = self.book_id(name).clone();
        match self.books.contains_key(&id) {
            true => self.books.get_mut(&id).unwrap(),
            false => {
                self.books.insert(id, Book {
                    id,
                    ..Default::default()
                });
                self.books.get_mut(&id).unwrap()
            }
        }
    }

    pub fn replace(&mut self, name: Label, book: Book) -> Option<Book> {
        let id = self.book_id(&name).clone();
        self.books.insert(id, book)
    }

    pub fn remove(&mut self, name: &Label) -> Option<Book> {
        let id = &self.book_id(&name).clone();
        self.books.remove(id)
    }

    pub fn rename(&mut self, old: &Label, new: String) -> Option<Book> {
        if self.titles.contains_key(old) {
            let key = self.titles.remove(old).unwrap();
            let b = self.books.remove(&key).unwrap();
            self.titles.insert(new.into(), b.id);
            self.books.insert(key, b)
        } else {
            None
        }
    }

    pub fn size(&self) -> usize { self.books.len() }

    fn book_id(&mut self, name: &Label) -> &Id {
        match self.titles.contains_key(&name) {
            true => self.titles.get(&name).unwrap(),
            false => {
                self.titles.insert(name.to_owned(), self.new_id());
                self.titles.get(&name).unwrap()
            }
        }
    }

    fn new_id(&self) -> Id { ID_COUNTER.fetch_add(1, Ordering::SeqCst) % Id::MAX }

    pub async fn add_book(
        &mut self, (title, bk): (&Label, Book),
    ) -> Option<Book> {
        let key = *self.book_id(title);
        self.books.insert(key, bk)
    }
}

impl<T: Into<String>> From<T> for Label {
    fn from(s: T) -> Self { Self(s.into()) }
}
