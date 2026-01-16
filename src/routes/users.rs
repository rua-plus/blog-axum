use anyhow::Context;
use axum::{Router, extract::State, routing::get};
use sqlx::PgPool;

use crate::error::AppResult;
use crate::models::User;
use crate::response::{StatusCode, SuccessResponse};

pub fn routes() -> Router<PgPool> {
    Router::new().route("/users/list", get(get_users_list))
}

async fn get_users_list(
    State(pool): State<PgPool>,
) -> AppResult<axum::response::Json<SuccessResponse<Vec<User>>>> {
    let users = sqlx::query_as::<_, User>(
r#"SELECT id, username, email, avatar_url, bio, last_login, created_at, updated_at FROM users
ORDER BY created_at DESC"#
    )
        .fetch_all(&pool)
        .await
        .context("Failed to query users")?;

    Ok(StatusCode::success(Some(users)).into())
}
