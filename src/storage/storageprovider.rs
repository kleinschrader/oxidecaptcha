mod memory;

use std::time::Duration;

pub use memory::MemoryStorage;

use crate::{challenge::Challenge, config::Config};

use super::Storage;

#[derive(Debug, Clone)]
pub enum StorageProvider {
    Memory(MemoryStorage),
}

impl Storage for StorageProvider {
    async fn get_site(&self, id: &uuid::Uuid) -> Option<crate::site::Site> {
        match self {
            StorageProvider::Memory(memory_storage) => memory_storage.get_site(id).await,
        }
    }

    async fn get_challange(
        &self,
        id: &uuid::Uuid,
        site: &crate::site::Site,
    ) -> Option<Challenge<'static, ()>> {
        match self {
            StorageProvider::Memory(memory_storage) => memory_storage.get_challange(id, site).await,
        }
    }

    async fn store_challenge(
        &self,
        site: &crate::site::Site,
        challenge: &Challenge<'static, ()>,
    ) -> Result<(), super::StorageError> {
        match self {
            StorageProvider::Memory(memory_storage) => {
                memory_storage.store_challenge(site, challenge).await
            }
        }
    }

    async fn delete_challenge(
        &self,
        site: &crate::site::Site,
        challenge: &Challenge<'static, ()>,
    ) -> Result<(), super::StorageError> {
        match self {
            StorageProvider::Memory(memory_storage) => {
                memory_storage.delete_challenge(site, challenge).await
            }
        }
    }
}

impl StorageProvider {
    pub fn new(config: &Config) -> Self {
        let config = config.get_storage();

        match config {
            crate::config::StorageTypeConfig::Memory(in_memory_config) => {
                let house_config = in_memory_config.get_house_keeping();
                let duration = Duration::from_secs(house_config.interval_seconds);
                StorageProvider::Memory(MemoryStorage::new(
                    in_memory_config.get_sites(),
                    duration,
                    house_config.batch_size,
                ))
            }
        }
    }
}
