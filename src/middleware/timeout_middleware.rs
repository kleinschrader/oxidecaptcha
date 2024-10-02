use std::{pin::pin, time::Duration};

use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse as _, Response},
};
use futures::FutureExt;
use tokio::select;

use crate::error_response::{ErrorId, ErrorResponse};

pub async fn timeout_middleware(request: Request, next: Next) -> Response {
    let timeout = async {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    .fuse();

    let response = pin!(next.run(request).fuse());

    let body_response;

    select! {
        response_res = response => body_response = response_res,
        () = timeout => {
            let e = ErrorResponse::new(ErrorId::Timeout, "Application has timed-out")
                .into_response();

            body_response = e;
        },
    };

    body_response
}
