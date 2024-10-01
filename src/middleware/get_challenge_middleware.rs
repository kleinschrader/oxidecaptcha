use axum::{extract::{Path, Request, State}, middleware::Next, response::Response};

use crate::{errorResponse::{ErrorId, ErrorResponse}, storage::Storage};

pub async fn get_challenge_middleware(
    State(state): State<crate::state::State>,
    Path((site_id, challenge_id)): Path<(String, String)>,
    mut request: Request,
    next: Next,
) -> Result<Response, ErrorResponse> {
    let site_id = site_id.parse()
        .map_err(|_| ErrorResponse::new(ErrorId::SiteNotFound, "Site not found"))?;

    let challenge_id = challenge_id.parse()
        .map_err(|_| ErrorResponse::new(ErrorId::ChallangeNotFound, "Challenge not Found"))?;

    let storage = state.get_storage().await;

    let site = storage.get_site(&site_id)
        .await
        .ok_or(ErrorResponse::new(ErrorId::SiteNotFound, "Site not found"))?;

    let challenge = storage
        .get_challange(&challenge_id, &site)
        .await
        .ok_or(ErrorResponse::new(ErrorId::ChallangeNotFound, "Challenge not Found"))?;

    request.extensions_mut().insert(site);
    request.extensions_mut().insert(challenge);

    let response = next.run(request).await;

    Ok(response)
}