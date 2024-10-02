use axum::{
    extract::{Path, Request, State},
    middleware::Next,
    response::Response,
};

use crate::{
    error_response::{ErrorId, ErrorResponse},
    storage::Storage,
};

pub async fn get_site_middleware(
    State(state): State<crate::state::State>,
    Path(site_id): Path<String>,
    mut request: Request,
    next: Next,
) -> Result<Response, ErrorResponse> {
    let site_id = site_id
        .parse()
        .map_err(|_| ErrorResponse::new(ErrorId::SiteNotFound, "Site not found"))?;

    let site = state
        .get_storage()
        .await
        .get_site(&site_id)
        .await
        .ok_or(ErrorResponse::new(ErrorId::SiteNotFound, "Site not found"))?;

    request.extensions_mut().insert(site);

    let response = next.run(request).await;

    Ok(response)
}
