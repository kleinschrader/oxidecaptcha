use std::fmt::Display;

use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use serde::Serialize;

#[derive(Debug)]
pub enum ErrorId {
    MissingApiKey,
    WrongApiKey,
    SiteNotFound,
    ChallangeNotFound,
    SolutionWrongSize,
    InternalServerError,
    Timeout,
}

impl From<ErrorId> for StatusCode {
    fn from(value: ErrorId) -> Self {
        match value {
            ErrorId::MissingApiKey => StatusCode::UNAUTHORIZED,
            ErrorId::WrongApiKey => StatusCode::UNAUTHORIZED,
            ErrorId::SiteNotFound => StatusCode::NOT_FOUND,
            ErrorId::ChallangeNotFound => StatusCode::NOT_FOUND,
            ErrorId::SolutionWrongSize => StatusCode::BAD_REQUEST,
            ErrorId::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorId::Timeout => StatusCode::SERVICE_UNAVAILABLE,
        }
    }
}

impl Serialize for ErrorId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ErrorId::MissingApiKey => serializer.serialize_str("MissingApiKey"),
            ErrorId::WrongApiKey => serializer.serialize_str("WrongApiKey"),
            ErrorId::SiteNotFound => serializer.serialize_str("SiteNotFound"),
            ErrorId::ChallangeNotFound => serializer.serialize_str("ChallengeNotFound"),
            ErrorId::SolutionWrongSize => serializer.serialize_str("SolutionWrongSize"),
            ErrorId::InternalServerError => serializer.serialize_str("InternalServerError"),
            ErrorId::Timeout => serializer.serialize_str("Timeout"),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    id: ErrorId,
    context: String,
}

impl ErrorResponse {
    pub fn new(id: ErrorId, context: impl Display) -> Self {
        let context = context.to_string();

        Self { id, context }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let body: Body = serde_json::to_string(&self)
            .expect("Unable to build json")
            .into();

        Response::builder()
            .header("Content-Type", "application/json")
            .status(StatusCode::from(self.id))
            .body(body)
            .expect("Unable to build response")
    }
}
