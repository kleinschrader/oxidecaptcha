use std::collections::BTreeMap;
use std::sync::Arc;

use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{challenge::Challenge, site::Site};

use crate::storage::{Storage, SiteNotFoundError};

#[derive(Debug, Clone)]
pub struct MemoryStorage(Arc<Mutex<MemoryStorageInner>>);

#[derive(Debug)]
struct MemoryStorageInner {
    sites: BTreeMap<Uuid, Site>,
    challanges: BTreeMap<(Uuid, Uuid), Challenge<'static, ()>>
}

impl MemoryStorage {
    pub fn new(sites: &Vec<Site>) -> Self {
        let challanges = BTreeMap::new();

        let sites: BTreeMap<Uuid, Site> = sites.into_iter()
            .map(|s| (s.get_id().to_owned(), s.to_owned()))
            .collect();

        let storage = MemoryStorageInner {
            challanges,
            sites
        };

        let storage = Mutex::new(storage);
        let storage = Arc::new(storage);

        Self(storage)
    }
}

impl Storage for MemoryStorage {
    async fn get_site(&self, id: &Uuid) -> Option<Site> {
        self.0.lock().await.sites.get(id).cloned()
    }

    async fn get_challange(&self, id: &Uuid, site: &Site) -> Option<Challenge<'static, ()>> {
        let site_id = site.get_id();
        let key = (site_id.to_owned(), id.to_owned());

        self.0.lock().await.challanges.get(&key).cloned()
    }

    async fn store_challenge(&mut self, site: &Site, challenge: Challenge<'static, ()>) -> Result<(), SiteNotFoundError> {
        let site_id = site.get_id();
        let challange_id = challenge.get_id();

        let mut lock = self.0.lock().await;
        if !lock.sites.contains_key(site_id) {
            return Err(SiteNotFoundError{})
        };

        let key = (site_id.to_owned(), challange_id.to_owned());

        lock.challanges.insert(key, challenge);

        Ok(())
    }
}