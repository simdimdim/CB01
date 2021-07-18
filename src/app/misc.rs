use crate::{Id, Library};
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct Bimap<K: Clone + Hash + Eq, V: Copy + Hash + Eq> {
    pub map: bimap::BiMap<K, V>,
}
impl<K, V> Default for Bimap<K, V>
where
    K: Clone + Hash + Eq,
    V: Copy + Hash + Eq + Into<K>,
{
    fn default() -> Self { Self::new() }
}
impl<K, V> Bimap<K, V>
where
    K: Clone + Hash + Eq,
    V: Copy + Hash + Eq + Into<K>,
{
    pub fn new() -> Self {
        let map = bimap::hash::BiHashMap::<K, V>::new();
        Self { map }
    }

    /// Adds a title if it doesn't exists and returns the Id associated with it,
    /// if it's already in the map returns the title id
    pub fn add_name<S>(&mut self, key: K) -> V
    where
        S: Into<V>,
        S: From<Id>, {
        if self.map.contains_left(&key) {
            *self.map.get_by_left(&key).unwrap()
        } else {
            let id = Library::new_id::<S>().into();
            self.map.insert_no_overwrite(key, id).ok();
            id
        }
    }

    /// Adds an Id if it doesn't exists and returns the title associated with it
    /// if it's already in the map returns the title
    pub fn add_id(&mut self, key: V) -> &K {
        if !self.map.contains_right(&key) {
            self.map.insert_no_overwrite(key.into(), key).ok();
        }
        self.map.get_by_right(&key).unwrap()
    }

    /// Returns the Id associated with a title if the title exists
    pub fn id(&self, key: &K) -> Option<&V> { self.map.get_by_left(key) }

    /// Returns the name associated with an Id if the Id exists
    pub fn title(&self, key: V) -> Option<K> {
        self.map.get_by_right(&key).map(K::to_owned)
    }

    /// returns true on success
    pub fn rename_by_title(&mut self, old: &K, new: K) -> bool {
        self.map
            .remove_by_left(old)
            .map(|(_, v)| self.map.insert(new, v))
            .is_some()
    }

    /// returns true on success
    pub fn rename_by_id(&mut self, old: V, new: K) -> bool {
        self.map
            .remove_by_right(&old)
            .map(|(_, v)| self.map.insert(new, v))
            .is_some()
    }
}
