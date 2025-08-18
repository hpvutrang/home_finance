use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::model;

pub trait Repository<K, V>
where
    K: Eq + std::hash::Hash + Copy,
{
    async fn get(&self, key: &K) -> Result<Option<V>, Box<dyn std::error::Error>>;
    async fn add(&self, key: K, value: V) -> Result<(), Box<dyn std::error::Error>>;
}

type MapCache<K, V> = Arc<Mutex<HashMap<K, V>>>;

pub struct AccountRepository {
    cache: MapCache<i32, model::account::Account>,
}

impl AccountRepository {
    pub fn new() -> Self {
        AccountRepository {
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Repository<i32, model::account::Account> for AccountRepository {
    async fn get(
        &self,
        key: &i32,
    ) -> Result<Option<model::account::Account>, Box<dyn std::error::Error>> {
        let cache = self.cache.lock().unwrap();
        Ok(cache.get(key).cloned())
    }

    async fn add(
        &self,
        key: i32,
        value: model::account::Account,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cache = self.cache.lock().unwrap();
        cache.insert(key, value);
        Ok(())
    }
}

impl AccountRepository {
    pub async fn get_id_by_name(
        &self,
        name: &str,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let cache = self.cache.lock().unwrap();
        for (key, value) in cache.iter() {
            if value.name == name {
                return Ok(*key);
            }
        }
        Err("Account not found".into())
    }
}