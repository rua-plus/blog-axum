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
use crate::utils::password;

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/users/list", get(get_users_list))
        .route("/users/create", post(create_user))
        .route("/users/login", post(login))
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
    let password_hash = password::hash_password(&payload.password)?;

    let user = sqlx::query_as::<_, User>(
        r#"INSERT INTO users (username, email, avatar_url, bio, password_hash)
        VALUES ($1, $2, NULL, NULL, $3)
        RETURNING id, username, email, avatar_url, bio, last_login, created_at, updated_at"#,
    )
    .bind(&payload.username)
    .bind(&payload.email)
    .bind(&password_hash)
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

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String,
}

async fn login(
    State(pool): State<PgPool>,
    ValidatedJson(payload): ValidatedJson<LoginRequest>,
) -> AppResult<axum::response::Json<SuccessResponse<User>>> {
    let user: User = sqlx::query_as(
        r#"SELECT id, username, email, avatar_url, bio, last_login, created_at, updated_at
        FROM users WHERE email = $1"#,
    )
    .bind(&payload.email)
    .fetch_one(&pool)
    .await
    .context("Invalid email or password")?;

    let password_hash: String =
        sqlx::query_scalar(r#"SELECT password_hash FROM users WHERE email = $1"#)
            .bind(&payload.email)
            .fetch_one(&pool)
            .await
            .context("Invalid email or password")?;

    password::verify_password(&payload.password, &password_hash)?;

    Ok(StatusCode::success(Some(user)).into())
}
