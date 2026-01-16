use anyhow::Context;
use axum::{
    Router,
    extract::State,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use validator::Validate;

use crate::error::AppResult;
use crate::extractors::ValidatedJson;
use crate::models::User;
use crate::response::{StatusCode, SuccessResponse};

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/users/list", get(get_users_list))
        .route("/users/create", post(create_user))
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String,
}

async fn create_user(
    State(pool): State<PgPool>,
    ValidatedJson(payload): ValidatedJson<CreateUserRequest>,
) -> AppResult<axum::response::Json<SuccessResponse<User>>> {
    let user = sqlx::query_as::<_, User>(
        r#"INSERT INTO users (username, email, avatar_url, bio, password_hash)
        VALUES ($1, $2, NULL, NULL, $3)
        RETURNING id, username, email, avatar_url, bio, last_login, created_at, updated_at"#,
    )
    .bind(&payload.username)
    .bind(&payload.email)
    .bind(&payload.password)
    .fetch_one(&pool)
    .await
    .context("Failed to create user")?;

    Ok(StatusCode::created(Some(user)).into())
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
