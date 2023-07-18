pub mod models;

use std::{collections::HashMap, hash::Hash};

pub struct Memory<K, V>
where
    K: Eq + PartialEq + Hash,
{
    items: HashMap<K, V>,
}

impl<K, V> Memory<K, V>
where
    K: Eq + PartialEq + Hash,
{
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: K, value: V) {
        self.items.insert(key, value);
    }

    pub fn remove(&mut self, key: K) {
        self.items.remove(&key);
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        self.items.get(key)
    }
}
