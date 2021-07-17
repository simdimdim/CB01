use crate::{Chapter, Content, Id, Label, Page};
use itertools::{Either, Itertools};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{
    collections::{btree_map::Range, BTreeMap},
    ops::RangeInclusive,
    path::PathBuf,
};

pub enum Position {
    First,
    BeforeCurrent,
    AfterCurrent,
    Last,
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Book {
    pub src:      Option<Page>,
    pub content:  BTreeMap<u16, Content>,
    pub chapters: Vec<Chapter>,
}
impl Book {
    pub fn new(page: Option<Page>) -> Self {
        let mut content = BTreeMap::new();
        // 0th index content to be used as cover page, to be chagned to default
        content.insert(0, Content::Empty);
        // 0th index chapter to be used as bookmark
        Self {
            src: page,
            content,
            chapters: vec![Chapter::default()],
        }
    }

    pub fn open<T: Into<Label>>(label: T, pb: PathBuf) -> (Label, Book) {
        let title: Label = label.into();
        let pb = pb.join(&title.0);
        let mut book = Book::default();
        book.cont_add(
            pb.read_dir()
                .expect("read_dir call failed")
                .flat_map(|d| d)
                .fold(vec![], |mut acc, d| {
                    d.path().extension().map(|f| {
                        if f == "jpg" {
                            acc.push(Content::Image {
                                pb:  d.path(),
                                src: None,
                            });
                        }
                    });
                    acc
                }),
            None,
        );
        (title, book)
    }

    pub fn get_cover(&self) -> &Content { self.content.get(&0).unwrap() }

    pub fn last(&self) -> &Chapter { &self.chapters[0] }

    pub fn chapter(&self, n: Id) -> Option<Range<Id, Content>> {
        self.valid(n as usize)
            .then(|| self.cont_batch(self.chapters[n as usize].range()))
    }

    pub fn chap_info(&self, n: Id) -> Option<Chapter> {
        (self.chapters.len() < n as usize)
            .then(|| self.chapters[n as usize].clone())
    }

    pub fn chap_swap(&mut self, n1: Id, n2: Id) {
        if self.valid(n1 as usize) && self.valid(n2 as usize) && n1 != n2 {
            self.chapters.swap(n1 as usize, n2 as usize)
        }
    }

    pub fn chap_add(&mut self, n1: Option<usize>, l: usize) -> &mut Chapter {
        let prev;
        match n1 {
            Some(n) => {
                if self.valid(n) {
                    prev = self.chapters[n].end() + 1;
                    self.chapters.push((prev, l as Id).into());
                }
            }
            None => {
                prev = self
                    .chapters
                    .par_iter()
                    .max_by_key(|c| c.end())
                    .map(|c| c.end())
                    .unwrap_or(0) +
                    1;
                self.chapters.push((prev, l as Id).into());
            }
        }
        self.chapters.last_mut().unwrap()
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
        if self.content.contains_key(&n1) &&
            self.content.contains_key(&n2) &&
            n1 != n2
        {
            let a = self.content.get_mut(&n1).unwrap() as *mut _;
            let b = self.content.get_mut(&n2).unwrap() as *mut _;
            unsafe {
                std::ptr::swap(a, b);
            }
        }
    }

    pub fn key_max(
        rng: Either<&BTreeMap<Id, Content>, Range<Id, Content>>,
    ) -> Id {
        match rng {
            Either::Left(a) => *a.par_iter().max_by_key(|(&k, _)| k).unwrap().0,
            Either::Right(b) => *b.max_by_key(|(&k, _)| k).unwrap().0,
        }
    }

    pub fn cont_add(&mut self, cont: Vec<Content>, pos: Option<Position>) {
        let default = (&(1 as Id), &Content::Empty);
        let split = match pos.unwrap_or(Position::Last) {
            Position::First => 1,
            Position::BeforeCurrent => 1.max(self.chapters[0].start() as usize),
            Position::AfterCurrent => self.chapters[0].end() as usize + 1,
            Position::Last => self.cont_len() as usize,
        } as Id;
        let len = cont.len() as Id;
        let sp = self.content.len() > 1;

        // lengthen chapters
        // filtered by insert pos
        // or better conditional lengthen depending on insert pos

        if sp {
            let first = self
                .content
                .range(split..)
                .next()
                .unwrap_or(self.content.iter().rev().next().unwrap())
                .0
                .clone();
            let mut leftovers = self.content.split_off(&first);
            let l = self
                .content
                .par_iter()
                .max_by_key(|(&k, _)| k)
                .unwrap_or(default)
                .0
                .clone() +
                1;
            cont.into_iter().enumerate().for_each(|(n, content)| {
                self.content.insert(l + n as Id, content);
            });
            self.content.append(
                &mut leftovers
                    .iter_mut()
                    .map(|(k, v)| ((k + len), v.clone()))
                    .collect::<BTreeMap<Id, Content>>(),
            );
        } else {
            cont.into_iter().enumerate().for_each(|(n, content)| {
                self.content.insert(n as Id + 1, content);
            });
        }
    }

    pub fn cont_remove(&mut self, _new: Vec<Content>) { todo!() }

    pub fn cont_len(&self) -> usize { self.content.len() }

    fn valid(&self, n: usize) -> bool { self.chap_len() > n && n > 0 }

    pub fn save(&self, _pb: PathBuf) { self.content.iter(); }

    pub fn current(&self) -> Range<Id, Content> {
        self.cont_batch(self.chapters[0].range())
    }

    pub fn chap_set_len(&mut self, ch: usize, l: Option<Id>) -> &mut Self {
        let idx = if self.valid(ch) { ch } else { 0 };
        l.map(|n| self.chapters[idx].len = n.saturating_sub(1));
        self
    }

    pub fn advance_by(&mut self, n: Id) -> Range<Id, Content> {
        self.chapters[0].offset = (self.cont_len() as Id)
            .min(self.chapters[0].offset.saturating_add(n));
        self.cont_batch(self.last().range())
    }

    pub fn backtrack_by(&mut self, n: Id) -> Range<Id, Content> {
        self.chapters[0].offset = self.chapters[0].offset.saturating_sub(n);
        self.cont_batch(self.last().range())
    }
}

impl Default for Book {
    fn default() -> Self { Self::new(None) }
}
impl Ord for Book {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.src.cmp(&other.src) }
}
impl PartialOrd for Book {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.src.cmp(&other.src))
    }
}
