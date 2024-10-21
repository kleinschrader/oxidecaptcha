use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Extension,
};

use crate::{
    error_response::{ErrorId::SiteNotFound, ErrorResponse},
    site::Site,
    Storage,
};

pub async fn get_challange(
    State(state): State<crate::State>,
    Extension(site): Extension<Site>,
) -> Result<Response, ErrorResponse> {
    let challenge = site.generate_challenge();

    state
        .get_storage()
        .await
        .store_challenge(&site, &challenge)
        .await
        .map_err(|_| ErrorResponse::new(SiteNotFound, "Site not found"))?;

    Ok(challenge.into_response())
}
