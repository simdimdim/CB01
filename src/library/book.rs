use super::{Chapter, Content};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{
    collections::{btree_map::Range, BTreeMap},
    ops::RangeInclusive,
    path::PathBuf,
};

pub(crate) type Id = u16;

#[derive(Debug, Clone)]
pub struct Book {
    pub id:       u16,
    pub content:  BTreeMap<u16, Content>,
    pub chapters: Vec<Chapter>,
    pub last:     Chapter,
}

impl Book {
    pub fn new() -> Self {
        let mut content = BTreeMap::new();
        content.insert(0, Content::Empty);
        Self {
            id: Id::MAX,
            content,
            chapters: Vec::new(),
            last: Chapter::default(),
        }
    }

    pub fn open<T: Into<String>>(label: T) -> Book {
        let _pb = PathBuf::from("library").join(label.into());
        Book::default()
    }

    pub fn chapter(&self, n: Id) -> Option<Range<Id, Content>> {
        (self.chapters.len() < n as usize)
            .then(|| self.cont_batch(self.chapters[n as usize].range()))
    }

    pub fn chap_info(&self, n: Id) -> Option<Chapter> {
        (self.chapters.len() < n as usize)
            .then(|| self.chapters[n as usize].clone())
    }

    pub fn chap_swap(&mut self, n1: Id, n2: Id) {
        self.valid(n1.max(n2) as usize)
            .then(|| self.chapters.swap(n1 as usize, n2 as usize));
    }

    pub fn chap_add(&mut self, n1: Option<usize>, l: usize) {
        let prev;
        match n1 {
            Some(n) => {
                if self.valid(n) {
                    prev = self.chapters[n].end();
                    self.chapters.push((prev + 1, prev + l as u16).into());
                }
            }
            None => {
                prev = self
                    .chapters
                    .par_iter()
                    .max_by_key(|c| c.end())
                    .map(|c| c.end())
                    .unwrap_or(0);
                self.chapters.push((prev + 1, prev + l as u16).into());
            }
        }
    }

    pub fn chap_remove(&mut self, n: usize) -> Option<(Chapter, Vec<Content>)> {
        self.valid(n).then(|| {
            let ch = self.chapters.remove(n);
            // let cnt = self
            //     .content
            //     .drain_filter(|k, _| ch.range().contains(k))
            //     .map(|(_, v)| v)
            //     .collect();
            let mut cnt = vec![];
            self.content.retain(|k, v| {
                ch.contains(k).then(|| cnt.push(v.to_owned())).is_none()
            });
            (ch, cnt)
        })
    }

    pub fn chap_len(&self) -> usize { self.chapters.len() }

    pub fn content(&self, n: &Id) -> Option<&Content> { self.content.get(n) }

    pub fn cont_batch(&self, range: RangeInclusive<Id>) -> Range<Id, Content> {
        self.content.range(range)
    }

    pub fn cont_swap(&mut self, n1: Id, n2: Id) {
        self.valid(n1.max(n2) as usize)
            .then(|| self.chapters.swap(n1 as usize, n2 as usize));
    }

    pub fn cont_add(&mut self, _new: Vec<Content>) { todo!() }

    pub fn cont_remove(&mut self, _new: Vec<Content>) { todo!() }

    pub fn cont_len(&self) -> usize { self.content.len() }

    fn valid(&self, n: usize) -> bool { self.chap_len() < n }
}

impl Default for Book {
    fn default() -> Self { Self::new() }
}
impl From<Id> for Book {
    fn from(id: Id) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}
