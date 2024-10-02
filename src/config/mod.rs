use std::net::SocketAddr;

mod inmemoryconfig;

pub use inmemoryconfig::InMemoryConfig;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum StorageTypeConfig {
    Memory(InMemoryConfig),
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "listenSocket")]
    listen_socket: SocketAddr,
    storage: StorageTypeConfig,
}

impl Config {
    pub fn get_storage(&self) -> &StorageTypeConfig {
        &self.storage
    }

    pub fn get_listen_socket(&self) -> &SocketAddr {
        &self.listen_socket
    }
}
