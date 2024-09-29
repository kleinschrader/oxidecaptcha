use serde::Deserialize;
use uuid::Uuid;

use crate::site::Site;


#[derive(Debug, Deserialize)]
pub struct InMemoryConfig {
    sites: Vec<Site>
}


impl InMemoryConfig {
    pub fn get_sites(&self) -> &Vec<Site> {
        &self.sites
    }
}