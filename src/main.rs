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
        .layer(middleware::from_fn(middlewares::request_id_middleware))
        .layer(TraceLayer::new_for_http());

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
