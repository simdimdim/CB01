use crate::Bimap;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    sync::atomic::{AtomicU16, Ordering},
};

pub mod book;
pub mod chapter;
pub mod content;
pub use book::*;
pub use chapter::*;
pub use content::*;

pub(super) static ID_COUNTER: AtomicU16 = AtomicU16::new(0);

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Label(pub String);
#[derive(Debug, Clone)]
pub struct Library {
    pub titles: Bimap<Label, Id>,
    pub books:  BTreeMap<Id, Book>,
    pub groups: HashMap<String, HashSet<Id>>,
    pub cur:    Id,
}

impl Library {
    pub fn current(&mut self) -> &mut Book {
        self.books.get_mut(&self.cur).unwrap()
    }

    pub fn book(&mut self, name: &Label) -> &Book {
        let id = self.book_id(name).clone();
        match self.books.contains_key(&id) {
            true => self.books.get(&id).unwrap(),
            false => {
                self.books.insert(id, Book {
                    ..Default::default()
                });
                self.books.get(&id).unwrap()
            }
        }
    }

    pub fn book_by_id(&mut self, id: Id) -> &Book { self.books.get(&id).unwrap() }

    pub fn book_mut(&mut self, name: &Label) -> &mut Book {
        let id = self.book_id(name).clone();
        match self.books.contains_key(&id) {
            true => self.books.get_mut(&id).unwrap(),
            false => {
                self.books.insert(id, Book {
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

    /// true on success
    pub fn rename(&mut self, old: &Label, new: &String) -> bool {
        // let newlabel = new.into();
        self.titles.rename_by_title(old, &new.into())
    }

    pub fn size(&self) -> usize { self.books.len() }

    fn book_id(&mut self, name: &Label) -> Id {
        match self.titles.id(name) {
            Some(n) => n,
            None => self.titles.add_name(name.to_owned()),
        }
    }

    pub fn new_id() -> Id { ID_COUNTER.fetch_add(1, Ordering::SeqCst) % Id::MAX }

    pub fn add_book(&mut self, title: &Label, bk: Book) -> Option<Book> {
        let key = self.book_id(title);
        self.books.insert(key, bk)
    }

    pub fn add_batch_to_group(&mut self, name: &String, books: Vec<Id>) {
        books.into_iter().for_each(|id| {
            self.groups.get_mut(name).unwrap().insert(id);
        });
    }

    pub fn remove_batch_from_group(&mut self, name: &String, books: Vec<Id>) {
        books.into_iter().for_each(|id| {
            self.groups.get_mut(name).unwrap().remove(&id);
        });
    }

    pub fn add_to_group(&mut self, name: &String, id: Id) {
        self.groups.get_mut(name).unwrap().insert(id);
    }

    pub fn remove_from_group(&mut self, name: &String, id: Id) {
        self.groups.get_mut(name).unwrap().remove(&id);
    }

    pub fn get_group(&self, name: &str) -> Option<Vec<&Book>> {
        self.groups.get(name).map(|r| {
            r.iter().fold(vec![], |mut acc, b| {
                if let Some(bk) = self.books.get(b) {
                    acc.push(bk);
                };
                acc
            })
        })
    }

    pub fn add_group(&mut self, name: String) -> &mut HashSet<Id> {
        self.groups.entry(name).or_default()
    }

    pub fn remove_group(&mut self, name: &String) -> Option<HashSet<Id>> {
        self.groups.remove(name)
    }
}

impl Default for Library {
    fn default() -> Self {
        let mut titles = Bimap::default();
        let k = titles.add_name("No Books.".into());
        let mut books = BTreeMap::new();
        books.insert(k, Book::new(None));
        let mut groups = HashMap::new();
        groups.insert("Reading".to_owned(), HashSet::new());
        Self {
            titles,
            books,
            groups,
            cur: k,
        }
    }
}

impl<T: Into<String>> From<T> for Label {
    fn from(s: T) -> Self { Self(s.into()) }
}
