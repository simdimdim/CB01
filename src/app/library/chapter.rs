use crate::{Id, Label};
use reqwest::Url;
use std::ops::RangeInclusive;

#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Chapter {
    pub offset: Id,
    pub len:    Id,
    pub src:    Option<Url>,
    pub name:   Option<Label>,
    pub full:   bool,
}
impl Chapter {
    pub fn contains(&self, n: &Id) -> bool {
        &self.offset <= n && n <= &self.end()
    }

    pub fn range(&self) -> RangeInclusive<Id> {
        self.offset..=self.offset + self.len
    }

    pub fn len(&self) -> usize { self.len as usize }

    pub fn is_empty(&self) -> bool { false }

    pub fn start(&self) -> Id { self.offset }

    pub fn end(&self) -> Id { self.offset.saturating_add(self.len) }

    pub fn shorten(&mut self, slice: RangeInclusive<Id>) -> Id {
        // if range is after the Chapter return
        if &self.end() < slice.start() {
            return self.len;
        }
        // if range is left of the Chapter move to the left
        if slice.end() < &self.start() {
            self.offset -= slice.count() as Id;
            return self.len;
        }
        // all 4 kinds of overlap:
        // self contains slice,
        // self contains the end of the slice
        // self contains the start of the slice
        // slice contains self
        let rng = self.range();
        self.len = match (rng.contains(slice.start()), rng.contains(slice.end()))
        {
            (true, true) => self.len.saturating_sub(slice.count() as Id),
            (false, true) => {
                self.offset =
                    self.offset.saturating_sub(rng.start() - slice.start());
                self.len.saturating_sub(slice.end() - rng.start() + 1)
            }
            (true, false) => {
                self.len.saturating_sub(rng.end() - slice.start() + 1)
            }
            (false, false) => 0,
        };
        self.len
    }

    pub fn lengthen(&mut self, slice: RangeInclusive<Id>) -> Id {
        // if range is after the Chapter return
        if &self.end() < slice.start() {
            return self.len;
        }
        // if range is left of the Chapter move to the right
        if slice.end() < &self.start() {
            self.offset += slice.count() as Id;
            return self.len;
        }
        // all 4 kinds of overlap:
        // self contains slice,
        // self contains the end of the slice
        // self contains the start of the slice
        // slice contains self
        let rng = self.range();
        self.len = match (rng.contains(slice.start()), rng.contains(slice.end()))
        {
            (true, true) => self.len.saturating_add(slice.count() as Id),
            (false, true) => {
                self.offset =
                    self.offset.saturating_add(rng.start() - slice.start());
                self.len.saturating_add(slice.end() - rng.start() + 1)
            }
            (true, false) => {
                self.len.saturating_add(rng.end() - slice.start() + 1)
            }
            (false, false) => self.len.saturating_add(slice.count() as Id),
        };
        self.len
    }

    pub fn set_src(&mut self, src: Option<Url>) -> &mut Self {
        self.src = src;
        self
    }

    pub fn set_name(&mut self, name: Option<Label>) -> &mut Self {
        self.name = name;
        self
    }
}
impl From<(Id, Id)> for Chapter {
    fn from(n: (Id, Id)) -> Self {
        Self {
            offset: n.0,
            len: n.1,
            ..Default::default()
        }
    }
}
impl From<(Id, Id, Option<Url>)> for Chapter {
    fn from(n: (Id, Id, Option<Url>)) -> Self {
        Self {
            offset: n.0,
            len: n.1,
            src: n.2,
            ..Default::default()
        }
    }
}
