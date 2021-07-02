use crate::{Id, Label};
use std::ops::RangeInclusive;

#[derive(Debug, Clone, Default)]
pub struct Chapter {
    pub offset: Id,
    pub len:    Id,
    pub name:   Option<Label>,
}
impl Chapter {
    pub fn contains(&self, n: &Id) -> bool {
        &self.offset <= n && &(self.offset + self.len) <= n
    }

    pub fn range(&self) -> RangeInclusive<Id> {
        self.offset..=self.offset + self.len
    }

    pub fn len(&self) -> usize { self.len as usize }

    pub fn end(&self) -> Id { self.offset.saturating_add(self.len) }
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
