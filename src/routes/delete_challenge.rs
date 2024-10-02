use axum::{extract::State, Extension};

use crate::{
    challenge::Challenge,
    error_response::{ErrorId, ErrorResponse},
    site::Site,
    Storage,
};

pub async fn delete_challange<'site>(
    State(state): State<crate::State>,
    Extension(site): Extension<Site>,
    Extension(challenge): Extension<Challenge<'static, ()>>,
) -> Result<(), ErrorResponse> {
    let store = state.get_storage().await;

    store
        .delete_challenge(&site, &challenge)
        .await
        .map_err(|e| match e {
            crate::storage::StorageError::SiteNotFoundError => {
                ErrorResponse::new(ErrorId::SiteNotFound, "Site not found")
            }
            crate::storage::StorageError::ChallengeNotFound => {
                ErrorResponse::new(ErrorId::ChallangeNotFound, "Challenge not found")
            }
        })
}
