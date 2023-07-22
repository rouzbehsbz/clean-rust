pub mod models;

use std::{collections::HashMap, hash::Hash, sync::RwLock};

// TODO: check wethear tokio RwLock is better here
pub struct Memory<K, V>
where
    K: Eq + PartialEq + Hash,
    V: Clone
{
    items: RwLock<HashMap<K, V>>,
}

impl<K, V> Memory<K, V>
where
    K: Eq + PartialEq + Hash,
    V: Clone
{
    pub fn new() -> Self {
        Self {
            items: RwLock::new(HashMap::new()),
        }
    }

    pub fn add(&self, key: K, value: V) {
        let mut writer = self.items.write().unwrap();

        writer.insert(key, value);
    }

    pub fn remove(&self, key: K) {
        let mut writer = self.items.write().unwrap();


        writer.remove(&key);
    }

    pub fn get(&self, key: &K) -> Option<V> {
        let reader = self.items.read().unwrap();
         
        match reader.get(key) {
            Some(value) => Some(value.to_owned()),
            None => None
        }
    }
}
