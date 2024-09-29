use std::{error::Error, fmt::Display};

use uuid::Uuid;

use crate::{challenge::Challenge, site::Site};

mod storageprovider;
pub use storageprovider::StorageProvider;

#[derive(Debug)]
pub struct SiteNotFoundError {}

impl Display for SiteNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unable to find site")
    }
}

impl Error for SiteNotFoundError {}

pub trait Storage {
    async fn get_site(&self, id: &Uuid) -> Option<Site>;

    async fn get_challange(&self, id: &Uuid, site: &Site) -> Option<Challenge<'static, ()>>;

    async fn store_challenge(&mut self, site: &Site, challenge: Challenge<'static, ()>) -> Result<(), SiteNotFoundError>;
}