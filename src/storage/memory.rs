use std::collections::BTreeMap;

use uuid::Uuid;

use crate::{challenge::Challenge, site::Site};

use super::Storage;

#[derive(Debug)]
pub struct MemoryStorage {
    sites: BTreeMap<Uuid, Site>,
    challanges: BTreeMap<(Uuid, Uuid), Challenge>
}

impl MemoryStorage {
    pub fn new(sites: &Vec<Site>) -> Self {
        let challanges = BTreeMap::new();

        let sites: BTreeMap<Uuid, Site> = sites.into_iter()
            .map(|s| (s.get_id().to_owned(), s.to_owned()))
            .collect();

        Self {
            challanges,
            sites
        }
    }
}

impl Storage for MemoryStorage {
    async fn get_site(&self, id: &Uuid) -> Option<Site> {
        self.sites.get(id).cloned()
    }

    async fn get_challange(&self, id: &Uuid, site: &Site) -> Option<Challenge> {
        let site_id = site.get_id();
        let key = (site_id.to_owned(), id.to_owned());
        self.challanges.get(&key).cloned()
    }

    async fn store_challenge(&mut self, site: &Site, challenge: Challenge) -> Result<(), super::SiteNotFoundError> {
        let site_id = site.get_id();
        let challange_id = challenge.get_id();

        if !self.sites.contains_key(site_id) {
            return Err(super::SiteNotFoundError{})
        };

        let key = (site_id.to_owned(), challange_id.to_owned());

        self.challanges.insert(key, challenge);

        Ok(())
    }
}