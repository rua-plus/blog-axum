pub mod users;

use axum::Router;
use sqlx::PgPool;

pub fn create_routes() -> Router<(PgPool, crate::utils::jwt::JwtService)> {
    Router::new().merge(users::routes())
}
