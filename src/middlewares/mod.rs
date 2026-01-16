pub mod auth;

use axum::{Router, extract::Request, http::HeaderValue, middleware::Next, response::Response};
use tower_http::trace::TraceLayer;
use uuid::Uuid;

/// Middleware that adds a unique request ID to each request
///
/// This middleware generates a UUID for each incoming request and adds it
/// to the request headers as "X-Request-ID". This allows for tracking
/// individual requests through the system for debugging and logging purposes.
pub async fn request_id_middleware(mut request: Request, next: Next) -> Response {
    // Generate a new UUID for this request
    let request_id = Uuid::new_v4().to_string();

    // Add the request ID to the request headers
    let headers = request.headers_mut();
    headers.insert(
        "X-Request-ID",
        HeaderValue::from_str(&request_id).expect("Invalid header value"),
    );

    // Process the request
    let mut response = next.run(request).await;

    // Add the request ID to the response headers as well
    let response_headers = response.headers_mut();
    response_headers.insert(
        "X-Request-ID",
        HeaderValue::from_str(&request_id).expect("Invalid header value"),
    );

    response
}

/// Creates a TraceLayer for HTTP request/response logging
///
/// This layer provides comprehensive logging for HTTP requests and responses,
/// including request IDs, methods, paths, status codes, and latency.
pub fn build_trace_layer<S>(app: Router<S>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    app.layer(
        TraceLayer::new_for_http()
            .make_span_with(|request: &axum::http::Request<_>| {
                let request_id = request
                    .headers()
                    .get("X-Request-ID")
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("unknown");

                tracing::info_span!(
                    "http_request",
                    request_id = %request_id,
                    method = %request.method(),
                    path = %request.uri().path(),
                    version = ?request.version(),
                )
            })
            .on_request(|request: &axum::http::Request<_>, span: &tracing::Span| {
                let request_id = request
                    .headers()
                    .get("X-Request-ID")
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("unknown");

                tracing::info!(
                    parent: span,
                    request_id = %request_id,
                    headers = ?request.headers(),
                    "Request started"
                );
            })
            .on_response(
                |response: &axum::http::Response<_>,
                 latency: std::time::Duration,
                 span: &tracing::Span| {
                    tracing::info!(
                        parent: span,
                        status = %response.status(),
                        latency = ?latency,
                        headers = ?response.headers(),
                        "Response sent"
                    );
                },
            )
            .on_failure(
                |error: tower_http::classify::ServerErrorsFailureClass,
                 latency: std::time::Duration,
                 span: &tracing::Span| {
                    tracing::error!(
                        parent: span,
                        error = ?error,
                        latency = ?latency,
                        "Request failed"
                    );
                },
            ),
    )
}
