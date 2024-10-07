use axum::extract::State;

use crate::{error_response::{ErrorId, ErrorResponse}, storage::Storage};

pub async fn _health(
    State(state): State<crate::State>
) -> Result<(), ErrorResponse> {
    let storage_healthy = state.get_storage()
        .await
        .healthy()
        .await;

    if storage_healthy == false {
       return Err(ErrorResponse::new(ErrorId::InternalServerError, "Storage is not healthy"));
    }

    Ok(())
}