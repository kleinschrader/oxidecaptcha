use axum::{extract::State, Extension, Json};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use crate::{challenge::Challenge, error_response::{ErrorId, ErrorResponse}, site::Site, solution::Solution, storage::Storage};

#[derive(Debug, Deserialize)]
pub struct RequestBody {
    solutions: Vec<Option<Solution>>
}

#[derive(Debug, Serialize)]
pub struct ResponseBody{
    valid: bool
}

pub async fn validate_challenges(
    State(state): State<crate::State>,
    Extension(site): Extension<Site>,
    Extension(challenge): Extension<Challenge<'static, ()>>,
    Json(body): Json<RequestBody>
) -> Result<String, ErrorResponse> {
    let expected_prefix_count = site.get_prefix_count();
    let prefix_len = body.solutions.len();

    if prefix_len != expected_prefix_count {
        return Err(ErrorResponse::new(ErrorId::WrongNumberOfSolutions, format!("Expected {expected_prefix_count} solutions, got {prefix_len}")));
    }

    let mut valid_challenges: usize = 0;
    
    let difficulty = site.get_difficulty();

    let mut index = 0;
    for solution in body.solutions {
        if let Some(solution) = solution {
            let prefix = match challenge.get_prefix(index) {
                Some(v) => v,
                None => return Err(ErrorResponse::new(ErrorId::InternalServerError, "Internal challange was missing a prefix")),
            };

            let r = solution._validate(prefix, difficulty).await;

            if r {
                valid_challenges += 1;
            }
        }

        index += 1;
    }

    let valid = valid_challenges >= site.get_prefixes_to_solve();

    match state.get_storage().await.delete_challenge(&site, &challenge).await {
        Ok(_) => (),
        Err(_) => {
            info!("Challenge expired or got deleted while we were checking solution");
            return Err(ErrorResponse::new(ErrorId::ChallangeNotFound, "Challange not found"))
        },
    };

    serde_json::to_string( &ResponseBody{
        valid
    }).map_err(|e| {
        warn!("Unable to generate response {}", e);
        ErrorResponse::new(ErrorId::InternalServerError, "Unable to generate response")
    })
}