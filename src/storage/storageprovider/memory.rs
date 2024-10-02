use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use indexmap::IndexMap;
use rand::Rng;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tracing::info;
use uuid::Uuid;

use crate::{challenge::Challenge, site::Site};

use crate::storage::{Storage, StorageError};

#[derive(Debug, Clone)]
pub struct MemoryStorage {
    inner: Arc<Mutex<MemoryStorageInner>>,
    _housekeeper: Arc<JoinHandle<()>>,
}

#[derive(Debug)]
struct MemoryStorageInner {
    sites: BTreeMap<Uuid, Site>,
    challanges: IndexMap<(Uuid, Uuid), Challenge<'static, ()>>,
}

impl MemoryStorage {
    pub fn new(
        sites: &Vec<Site>,
        housekeeping_interval: Duration,
        housekeeping_batch_size: usize,
    ) -> Self {
        let challanges = IndexMap::new();

        let sites: BTreeMap<Uuid, Site> = sites
            .into_iter()
            .map(|s| (s.get_id().to_owned(), s.to_owned()))
            .collect();

        let storage = MemoryStorageInner { challanges, sites };

        let storage = Mutex::new(storage);
        let storage = Arc::new(storage);

        let storage_for_handle = storage.clone();

        let handle = tokio::task::spawn(async move {
            let housekeeping_interval = housekeeping_interval;
            let storage = storage_for_handle;
            let housekeeping_batch_size = housekeeping_batch_size;
            loop {
                tokio::time::sleep(housekeeping_interval.clone()).await;

                let locktime = Instant::now();

                let mut lock = storage.lock().await;

                let mut os_gen = rand::rngs::OsRng::default();

                let len = lock.challanges.len();

                if len == 0 {
                    continue;
                }

                let indicies_to_remove: Vec<usize> = (0..housekeeping_batch_size)
                    .into_iter()
                    .map(|_| os_gen.gen_range(0..len))
                    .filter(|i| lock.challanges[i.clone()].is_expired())
                    .collect();

                let indicies_to_remove_count = indicies_to_remove.len();

                for index in indicies_to_remove {
                    lock.challanges.swap_remove_index(index);
                }

                if indicies_to_remove_count > 0 {
                    let duration = locktime.elapsed().as_micros();
                    info!(
                        "Housekeeping task removed {} entries in {}us",
                        indicies_to_remove_count, duration
                    );
                }
            }
        });

        let handle = Arc::new(handle);

        Self {
            inner: storage,
            _housekeeper: handle,
        }
    }
}

impl Storage for MemoryStorage {
    async fn get_site(&self, id: &Uuid) -> Option<Site> {
        self.inner.lock().await.sites.get(id).cloned()
    }

    async fn get_challange(&self, id: &Uuid, site: &Site) -> Option<Challenge<'static, ()>> {
        let site_id = site.get_id();
        let key = (site_id.to_owned(), id.to_owned());

        self.inner.lock().await.challanges.get(&key).cloned()
    }

    async fn store_challenge(
        &self,
        site: &Site,
        challenge: &Challenge<'static, ()>,
    ) -> Result<(), StorageError> {
        let site_id = site.get_id();
        let challange_id = challenge.get_id();

        let mut lock = self.inner.lock().await;
        if !lock.sites.contains_key(site_id) {
            return Err(StorageError::SiteNotFoundError);
        };

        let key = (site_id.to_owned(), challange_id.to_owned());

        lock.challanges.insert(key, challenge.to_owned());

        Ok(())
    }

    async fn delete_challenge(
        &self,
        site: &Site,
        challenge: &Challenge<'static, ()>,
    ) -> Result<(), StorageError> {
        let site_id = site.get_id();
        let challenge_id = challenge.get_id();

        let key = (site_id.to_owned(), challenge_id.to_owned());

        let mut lock = self.inner.lock().await;

        lock.challanges
            .swap_remove(&key)
            .map(|_| ())
            .ok_or(StorageError::ChallengeNotFound)
    }
}
