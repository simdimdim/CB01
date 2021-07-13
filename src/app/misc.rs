use super::ID_COUNTER;
use crate::{Id, Library};
use fxhash::FxBuildHasher;
use indexmap::IndexSet;
use std::hash::Hash;

#[derive(Debug, Clone, Default)]
pub struct Bimap<K: Hash + Eq, V: Hash + Eq> {
    pub first:  IndexSet<K, FxBuildHasher>,
    pub second: IndexSet<V, FxBuildHasher>,
}
impl<T: Clone + Hash + Eq> Bimap<T, Id>
where
    T: From<String>,
{
    /// Adds an title if it doesn't exists and returns the Id associated with it
    /// if it's already in the map returns the title
    pub fn add_name(&mut self, key: T) -> Id {
        match self.first.get_index_of(&key) {
            Some(n) => self.second[n],
            None => {
                self.first.insert(key);
                let new = Library::new_id();
                self.second.insert(new);
                new
            }
        }
    }

    /// Adds an Id if it doesn't exists and returns the title associated with it
    /// if it's already in the map returns the title
    pub fn add_id(&mut self, key: Id) -> T {
        match self.second.get_index_of(&key) {
            Some(n) => self.first[n].clone(),
            None => {
                self.second.insert(key);
                let new: T = key.to_string().into();
                self.first.insert(new.clone());
                new
            }
        }
    }

    /// Returns the Id associated with a title if the title exists
    pub fn id(&mut self, key: &T) -> Option<Id> {
        self.first.get_index_of(key).map(|n| self.second[n].clone())
    }

    /// Returns the name associated with an Id if the Id exists
    pub fn title(&mut self, key: Id) -> Option<T> {
        self.second
            .get_index_of(&key)
            .map(|n| self.first[n].clone())
    }

    /// returns true on success
    pub fn rename_by_title(&mut self, old: &T, new: &T) -> bool {
        match (!self.first.contains(new), self.first.contains(old)) {
            (true, true) => {
                self.first.insert(new.clone());
                self.first.swap_remove(old)
            }
            _ => false,
        }
    }

    /// returns true on success
    pub fn rename_by_id(&mut self, old: Id, new: &T) -> bool {
        match (!self.first.contains(new), self.first.len() < old as usize) {
            (true, true) => {
                self.first.insert(new.clone());
                self.first.swap_remove_index(old as usize).is_some()
            }
            _ => false,
        }
    }
}
