use anyhow::Context;
use axum::{Router, extract::State, middleware, routing::get};
use sqlx::PgPool;
use tracing::{debug, info};

use crate::response::{StatusCode, SuccessResponse};
use crate::utils::{config, init_tracing};

mod error;
mod extractors;
mod middlewares;
mod models;
mod response;
mod routes;
mod utils;

async fn root(_state: State<PgPool>) -> axum::response::Json<SuccessResponse<&'static str>> {
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

    // 创建数据库连接池
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        app_config.postgresql.user,
        app_config.postgresql.password,
        app_config.postgresql.host,
        app_config.postgresql.port,
        app_config.postgresql.database
    );

    info!("Connecting databse: {}", app_config.postgresql.host);
    let pool = PgPool::connect(&database_url)
        .await
        .with_context(|| "Failed to connect to database")?;

    info!("Database connection pool created successfully");

    // 创建路由
    let app = Router::new()
        .route("/", get(root))
        .merge(routes::create_routes())
        .with_state(pool);
    let app = Router::new().nest("/api", app);

    let app = middlewares::build_trace_layer(app)
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
