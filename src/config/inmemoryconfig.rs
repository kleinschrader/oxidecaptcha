use serde::Deserialize;
use uuid::Uuid;

use crate::site::Site;

#[derive(Debug, Deserialize)]
pub struct HousekeepingConfig {
    #[serde(rename= "intervalSeconds")]
    pub interval_seconds: u64,
    #[serde(rename= "batchSize")]
    pub batch_size: usize,
}

#[derive(Debug, Deserialize)]
pub struct InMemoryConfig {
    housekeeping: HousekeepingConfig,
    sites: Vec<Site>
}


impl InMemoryConfig {
    pub fn get_sites(&self) -> &Vec<Site> {
        &self.sites
    }

    pub fn get_house_keeping(&self) -> &HousekeepingConfig {
        &self.housekeeping
    }
}