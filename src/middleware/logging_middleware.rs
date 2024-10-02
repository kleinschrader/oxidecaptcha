use axum::{extract::Request, middleware::Next, response::Response};
use tracing::info;

pub async fn logging_middleware(request: Request, next: Next) -> Response {
    let execution_duration = tokio::time::Instant::now();

    let method = request.method().to_owned();

    let url = request.uri().path().to_owned();

    let response = next.run(request).await;

    let execution_duration = execution_duration.elapsed().as_millis();

    let statuscode = response.status();

    info!("[{statuscode}][{execution_duration}ms] {method} {url}");

    response
}
