use std::{error::Error, fmt::Display};

use uuid::Uuid;

use crate::{challenge::Challenge, site::Site};

mod storageprovider;
pub use storageprovider::StorageProvider;

#[derive(Debug)]
pub enum StorageError {
    SiteNotFoundError,
    ChallengeNotFound,
}

impl Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error = match &self {
            StorageError::SiteNotFoundError => "Site not found",
            StorageError::ChallengeNotFound => "Challenge not found",
        };

        f.write_str(error)
    }
}

impl Error for StorageError {}

pub trait Storage: Send + Sync {
    async fn get_site(&self, id: &Uuid) -> Option<Site>;

    async fn get_challange(&self, id: &Uuid, site: &Site) -> Option<Challenge>;

    async fn store_challenge(
        &self,
        site: &Site,
        challenge: &Challenge,
    ) -> Result<(), StorageError>;

    async fn delete_challenge(
        &self,
        site: &Site,
        challenge: &Challenge,
    ) -> Result<(), StorageError>;

    async fn healthy(&self) -> bool;
}
