use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};
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

    // Log the request with its ID
    // tracing::info!(
    //     request_id = %request_id,
    //     method = %request.method(),
    //     path = %request.uri().path(),
    //     "Incoming request"
    // );

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
