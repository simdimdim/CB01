use crate::{Bimap, };
use core::ops::Deref;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    path::PathBuf,
    sync::atomic::{AtomicU16, Ordering},
};

pub mod book;
pub mod chapter;
pub mod content;
pub use book::*;
pub use chapter::*;
pub use content::*;

pub(crate) type Id = u16;
type IdStaticType = AtomicU16;

pub(super) static ID_COUNTER: IdStaticType = IdStaticType::new(0);

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Label(pub String);
#[derive(Debug, Clone)]
pub struct Library {
    pub titles: Bimap<Label, Id>,
    pub books:  BTreeMap<Id, Book>,
    pub groups: HashMap<String, HashSet<Id>>,
}

impl Library {
    pub fn name(&mut self, id: Id) -> Label { self.titles.title(id).unwrap() }

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
        self.titles.rename_by_title(old, new.into())
    }

    pub fn size(&self) -> usize { self.books.len() }

    fn book_id(&mut self, name: &Label) -> Id {
        match self.titles.id(name) {
            Some(&n) => n,
            None => self.titles.add_name::<Id>(name.to_owned()),
        }
    }

    pub fn new_id<T>() -> T
    where
        T: From<Id>, {
        T::from(ID_COUNTER.fetch_add(1, Ordering::SeqCst) % Id::MAX)
    }

    pub fn add_book(&mut self, title: &Label, bk: Book) -> Option<Book> {
        let id = self.book_id(title);
        self.books.insert(id, bk)
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

    pub fn add_to_group(&mut self, name: &str, id: Id) {
        self.groups.get_mut(name).unwrap().insert(id);
    }

    pub fn remove_from_group(&mut self, name: &String, id: Id) {
        self.groups.get_mut(name).unwrap().remove(&id);
    }

    pub fn get_group(&self, name: &str) -> Option<Vec<Book>> {
        self.groups.get(name).map(|r| {
            r.iter().fold(vec![], |mut acc, b| {
                if let Some(bk) = self.books.get(b) {
                    acc.push(bk.to_owned());
                };
                acc
            })
        })
    }

    pub fn get_groups(&self) -> Vec<&String> { self.groups.keys().collect() }

    pub fn get_group_names(&self, name: &str) -> Option<Vec<Label>> {
        self.groups.get(name).map(|h| {
            h.into_iter()
                .filter_map(|&n| self.titles.title(n))
                .collect()
        })
    }

    pub fn group_size(&mut self, name: &str) -> usize {
        if let Some(hs) = self.groups.get(name) {
            hs.len()
        } else {
            0
        }
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
        let mut titles = Bimap::new();
        let mut books = BTreeMap::new();
        let (_, b) = Book::open("", PathBuf::from("."));
        books.insert(titles.add_name::<Id>("Runtime dir".into()), b);
        let mut groups = HashMap::new();
        groups.insert("Reading".to_owned(), HashSet::new());
        Self {
            titles,
            books,
            groups,
        }
    }
}

impl<T: ToString> From<T> for Label {
    fn from(s: T) -> Self { Self(s.to_string()) }
}
impl Deref for Label {
    type Target = String;

    fn deref(self: &'_ Self) -> &'_ Self::Target { &self.0 }
}
