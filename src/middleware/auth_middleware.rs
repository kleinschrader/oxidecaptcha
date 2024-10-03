use axum::{
    extract::Request,
    middleware::Next,
    response::Response, Extension,
};
use sha2::{Digest, Sha256};

use crate::{
    error_response::{ErrorId, ErrorResponse}, site::Site
};

pub async fn auth_middleware(
    Extension(site): Extension<Site>,
    request: Request,
    next: Next,
) -> Result<Response, ErrorResponse> {

    let key = request.headers().get("api-key")
        .ok_or(ErrorResponse::new(ErrorId::MissingApiKey, "Header api-key missing"))?;

    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    let keyhash = hasher.finalize().to_vec();

    if site.get_api_key_hash() != &keyhash {
        return Err(ErrorResponse::new(ErrorId::WrongApiKey, "Api-key wrong"));
    }

    if site.get_api_key().as_bytes() != key.as_bytes() {
        return Err(ErrorResponse::new(ErrorId::WrongApiKey, "Api-key wrong"));
    }

    let response = next.run(request).await;

    Ok(response)
}
