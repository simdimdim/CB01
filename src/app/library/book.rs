use crate::{page::Find, Chapter, Content, Id, Label, Page, Retriever};
use itertools::Either;
use log::{info, trace, warn};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{
    collections::{btree_map::Range, BTreeMap},
    ffi::OsStr,
    ops::RangeInclusive,
    path::PathBuf,
};

#[derive(Debug, Copy, Clone)]
pub enum Position {
    Last,
    AfterCurrent,
    BeforeCurrent,
    First,
    Cover,
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Book {
    pub src:      Option<Page>,
    pub content:  BTreeMap<u16, Content>,
    pub chapters: Vec<Chapter>,
    pub cur_ch:   usize,
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
            cur_ch: 0,
        }
    }

    pub fn open(pb: PathBuf) -> (Label, Book) {
        static FAIL_MSG: &str = "read_dir call failed";

        let title;
        let canon = pb.canonicalize().unwrap();
        if pb.is_dir() {
            title = canon.file_name().unwrap().to_str().unwrap();
        } else if pb.is_file() {
            title = canon.parent().unwrap().to_str().unwrap();
        } else {
            title = "";
        }
        let mut book = Book::default();
        let mut cover = pb.join(title);
        cover.set_extension("jpg");
        if cover.exists() {
            trace!("Has cover: {:?}", cover);
            book.cont_add(
                vec![Content::Image {
                    pb:  cover,
                    src: None,
                }],
                Some(Position::Cover),
            );
        }
        let dir = pb.read_dir().expect(FAIL_MSG);
        let mut firstlevel = vec![];
        let mut toplvl = dir.flatten().fold(vec![], |mut acc, d| {
            if let Some(Some("jpg" | "jpeg" | "png" | "bmp" | "gif" | "tiff")) =
                d.path().extension().map(OsStr::to_str)
            {
                acc.push(Content::Image {
                    pb:  d.path(),
                    src: None,
                });
            } else if d.path().is_dir() {
                firstlevel.push(d.path());
            }
            acc
        });
        toplvl.sort();
        trace!("{:?}", toplvl);
        book.cont_add(toplvl, None);
        for p in firstlevel {
            let mut dir = p.read_dir().expect(FAIL_MSG).flatten().fold(
                vec![],
                |mut cvec, item| {
                    let path = item.path();
                    if let Some((
                        true,
                        Some("jpg" | "jpeg" | "png" | "bmp" | "gif" | "tiff"),
                    )) =
                        path.extension().zip(path.file_name()).map(|(p1, p2)| {
                            (p2 != OsStr::new("cover"), OsStr::to_str(p1))
                        })
                    {
                        cvec.push(Content::Image {
                            pb:  item.path(),
                            src: None,
                        });
                    }
                    cvec
                },
            );
            dir.sort();
            trace!("{:?}", dir);
            book.chap_add_from_parts(None, dir.len());
            book.cont_add(dir, None);
        }
        book.chap_next();
        (title.into(), book)
    }

    pub fn cover(&self) -> &Content { &self.content[&0] }

    pub fn last(&self) -> &Chapter { &self.chapters[0] }

    pub fn chapter(&self, n: Id) -> Option<Range<'_, Id, Content>> {
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

    pub fn chap_add_from_parts(
        &mut self, n1: Option<usize>, l: usize,
    ) -> &mut Chapter {
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

    pub fn chap_add(&mut self, ch: Chapter, pos: Option<Position>) {
        let default = Position::Last;
        match pos.unwrap_or(default) {
            Position::Last => self.chapters.push(ch),
            Position::AfterCurrent => self
                .chapters
                .insert(self.cur_ch.saturating_add(1).min(self.chaps_len()), ch),
            Position::BeforeCurrent => self.chapters.insert(
                if self.valid(self.cur_ch.saturating_sub(1).max(1)) {
                    self.cur_ch.saturating_sub(1).max(1)
                } else {
                    match default {
                        Position::Last => self.chaps_len(),
                        Position::Cover => 0,
                        _ => 1,
                    }
                },
                ch,
            ),
            Position::First => self.chapters.insert(1, ch),
            Position::Cover => self.chapters[0] = ch,
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

    pub fn chaps_len(&self) -> usize { self.chapters.len() }

    pub fn content(&self, n: &Id) -> Option<&Content> { self.content.get(n) }

    pub fn cont_batch(
        &self, range: RangeInclusive<Id>,
    ) -> Range<'_, Id, Content> {
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
        rng: Either<&BTreeMap<Id, Content>, Range<'_, Id, Content>>,
    ) -> Id {
        match rng {
            Either::Left(a) => *a.par_iter().max_by_key(|(&k, _)| k).unwrap().0,
            Either::Right(b) => *b.max_by_key(|(&k, _)| k).unwrap().0,
        }
    }

    pub fn cont_add(&mut self, cont: Vec<Content>, pos: Option<Position>) {
        if cont.is_empty() {
            return;
        }
        let len = cont.len() as Id;
        let mut cont = cont.into_iter();
        let default = (&1u16, &Content::Empty);
        let split = match pos.unwrap_or(Position::Last) {
            Position::Last => self.cont_len() as usize,
            Position::AfterCurrent => self.chapters[0].end() as usize + 1,
            Position::BeforeCurrent => 1.max(self.chapters[0].start() as usize),
            Position::First => 1,
            Position::Cover => {
                self.content
                    .entry(0)
                    .and_modify(|e| *e = cont.next().unwrap());
                if len == 1 {
                    return;
                };
                1
            }
        } as Id;

        // lengthen chapters
        // filtered by insert pos
        // or better conditional lengthen depending on insert pos

        match self.content.len() {
            2..=usize::MAX => {
                let first = *self
                    .content
                    .range(split..)
                    .next()
                    .unwrap_or_else(|| self.content.iter().rev().next().unwrap())
                    .0;
                let mut leftovers = self.content.split_off(&first);
                let l = *self
                    .content
                    .par_iter()
                    .max_by_key(|(&k, _)| k)
                    .unwrap_or(default)
                    .0 +
                    1;
                cont.enumerate().for_each(|(n, content)| {
                    self.content.insert(l + n as Id, content);
                });
                self.content.append(
                    &mut leftovers
                        .iter_mut()
                        .map(|(k, v)| ((k + len), v.clone()))
                        .collect::<BTreeMap<Id, Content>>(),
                );
            }
            1 => {
                trace!("Only a cover exists.");
                cont.enumerate().for_each(|(n, content)| {
                    self.content.insert(n as Id + 1, content);
                });
            }
            0 => {
                let cnt = cont.next().unwrap();
                self.content.insert(0, cnt.clone());
                self.content.insert(1, cnt);
                cont.enumerate().for_each(|(n, content)| {
                    self.content.insert(n as Id + 2, content);
                });
            }
            _ => unreachable!(),
        }
    }

    pub fn cont_remove(&mut self, _new: Vec<Content>) { todo!() }

    pub fn cont_len(&self) -> usize { self.content.len() }

    fn valid(&self, n: usize) -> bool { self.chaps_len() > n && n > 0 }

    pub fn save(&self, _pb: PathBuf) { self.content.iter(); }

    pub fn current(&self) -> Range<'_, Id, Content> {
        self.cont_batch(self.chapters[0].range())
    }

    pub fn chap_set_len(&mut self, ch: usize, len: Option<Id>) -> &mut Self {
        let idx = if self.valid(ch) { ch } else { 0 };
        if let Some(n) = len {
            self.chapters[idx].len =
                n.saturating_sub(1).min(self.chap_cur().end());
        }
        self
    }

    pub fn advance_by(&mut self, n: Id) -> Range<'_, Id, Content> {
        let adv = self.last_pos().saturating_add(n).min(self.cont_len() as Id);
        if self.chap_cur().contains(&adv) {
            self.chapters[0].offset = adv;
        } else if self.chap_cur().end() < adv &&
            self.valid(self.cur_ch.saturating_add(1))
        {
            self.chap_next();
            self.chapters[0].offset = self.cur_beg();
        }
        self.cont_batch(self.last().range())
    }

    pub fn backtrack_by(&mut self, n: Id) -> Range<'_, Id, Content> {
        let back = 1.max(self.last_pos().saturating_sub(n));
        if self.chap_cur().contains(&back) {
            self.chapters[0].offset = back;
        } else if back < self.cur_beg() && 1 < self.cur_ch.saturating_sub(1) {
            self.chap_prev();
            self.chapters[0].offset = self.cur_beg();
        } else if 1 <= self.cur_ch.saturating_sub(1) {
            self.chapters[0].offset = self.cur_beg();
        }
        self.cont_batch(self.last().range())
    }

    pub fn chap_cur(&self) -> &Chapter { &self.chapters[self.cur_ch] }

    pub fn last_pos(&self) -> Id { self.chapters[0].offset }

    pub fn cur_beg(&self) -> Id { self.chap_cur().offset }

    pub fn cur_end(&self) -> Id { self.chap_cur().end() }

    pub fn chap_next(&mut self) {
        let next = self.cur_ch.saturating_add(1);
        if self.valid(next) {
            self.cur_ch = next;
        }
    }

    pub fn chap_prev(&mut self) {
        let prev = self.cur_ch.saturating_sub(1);
        if self.valid(prev) {
            self.cur_ch = prev;
        }
    }

    pub fn is_last(&self) -> bool {
        self.content
            .range(self.chapters[self.cur_ch].end()..)
            .into_iter()
            .count() ==
            1
    }

    pub fn chap_sort_url(&mut self) {
        let cur = self.chap_cur().clone();
        self.chapters[1..].sort_by(|k1, k2| k1.src.cmp(&k2.src));
        if let Some(n) = self.chapters.iter().position(|c| c == &cur) {
            self.cur_ch = n;
        };
    }

    pub async fn next_by_last(
        &self, find: Find<'_>, ret: &Retriever,
    ) -> Option<Page> {
        if let Some(page) = self.last().src.clone().map(Page::from) {
            return ret.get(page).await.next(find).await;
        }
        None
    }

    pub async fn next_by_index(
        &self, find: Find<'_>, ret: Option<&Retriever>,
    ) -> Option<Page> {
        if let (Some(page), None) = (self.src.as_ref(), ret) {
            return page.links(find).await.pop();
        } else if let (Some(page), Some(r)) = (self.src.clone(), ret) {
            return r.get(page).await.links(find).await.pop();
        };
        None
    }

    pub async fn next_by_hand(
        &self, ret: &Retriever, url: String,
    ) -> Option<Page> {
        if let Ok(url) = url.parse::<url::Url>() {
            return Some(ret.get(url.into()).await);
        }
        None
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
