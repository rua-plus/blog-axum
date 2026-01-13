use anyhow::Context;
use axum::{Router, routing::get};
use tracing::{debug, info};

use crate::utils::init_tracing;

mod utils;

async fn root() -> &'static str {
    "RUA"
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing()?;

    // 获取 git 版本信息
    let git_version = option_env!("GIT_VERSION").unwrap_or("unknown");
    info!("Git Version: {}", git_version);
    debug!("Git Version: {}", git_version);

    // 创建路由
    let app = Router::new().route("/", get(root));

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
