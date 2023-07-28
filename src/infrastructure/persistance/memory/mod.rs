pub mod repositories;

use std::{collections::HashMap, hash::Hash};

use tokio::sync::RwLock;

pub struct Memory<K, V>
where
    K: Eq + PartialEq + Hash,
    V: Clone,
{
    items: RwLock<HashMap<K, V>>,
}

//TODO: make better argument typing (T, &T)
impl<K, V> Memory<K, V>
where
    K: Eq + PartialEq + Hash,
    V: Clone,
{
    pub fn new() -> Self {
        Self {
            items: RwLock::new(HashMap::new()),
        }
    }

    pub async fn add(&self, key: K, value: V) {
        let mut writer = self.items.write().await;

        writer.insert(key, value);
    }

    pub async fn remove(&self, key: K) {
        let mut writer = self.items.write().await;

        writer.remove(&key);
    }

    pub async fn get(&self, key: &K) -> Option<V> {
        let reader = self.items.read().await;

        match reader.get(key) {
            Some(value) => Some(value.to_owned()),
            None => None,
        }
    }

    pub async fn get_all(&self) -> Vec<V> {
        let reader = self.items.read().await;

        let values: Vec<V> = reader.values().cloned().collect();

        values
    }
}
