use std::time::Duration;

mod inmemoryconfig;

pub use inmemoryconfig::InMemoryConfig;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum StorageTypeConfig {
    Memory(InMemoryConfig)
}

#[derive(Debug, Deserialize)]
pub struct Config {
    storage: StorageTypeConfig
}

impl Config {
    pub fn get_storage(&self) -> &StorageTypeConfig {
        &self.storage
    }
}