use anyhow::Context;
use axum::{Router, middleware, routing::get};
use tower_http::trace::TraceLayer;
use tracing::{debug, info};

use crate::response::{StatusCode, SuccessResponse};
use crate::utils::{config, init_tracing};

mod middlewares;
mod response;
mod utils;

async fn root() -> axum::response::Json<SuccessResponse<&'static str>> {
    StatusCode::success(Some("RUA")).into()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing()?;

    // 加载配置
    let app_config = config::AppConfig::load()?;
    info!("Configuration loaded successfully: {:?}", app_config);

    // 获取 git 版本信息
    let git_version = option_env!("GIT_VERSION").unwrap_or("unknown");
    info!("Git Version: {}", git_version);
    debug!("Git Version: {}", git_version);

    // 创建路由
    let app = Router::new()
        .route("/", get(root))
        .layer(
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
        .layer(middleware::from_fn(middlewares::request_id_middleware));

    // 启动服务器
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .with_context(|| "Failed to bind TCP listener to 0.0.0.0:8000")?;
    info!("Server running on http://0.0.0.0:8000");
    axum::serve(listener, app)
        .await
        .with_context(|| "Failed to serve HTTP server")?;

    Ok(())
}
