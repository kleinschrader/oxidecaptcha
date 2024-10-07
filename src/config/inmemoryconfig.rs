use kale_duration::AbsoluteDuration;
use serde::Deserialize;

use crate::site::Site;

#[derive(Debug, Deserialize)]
pub struct HousekeepingConfig {
    pub interval: AbsoluteDuration,
    #[serde(rename = "batchSize")]
    pub batch_size: usize,
}

#[derive(Debug, Deserialize)]
pub struct InMemoryConfig {
    housekeeping: HousekeepingConfig,
    sites: Vec<Site>,
}

impl InMemoryConfig {
    pub fn get_sites(&self) -> &Vec<Site> {
        &self.sites
    }

    pub fn get_house_keeping(&self) -> &HousekeepingConfig {
        &self.housekeeping
    }
}
