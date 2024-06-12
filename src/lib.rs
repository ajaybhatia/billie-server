use std::sync::Arc;

use axum::{
    routing::{delete, get, post},
    Router,
};

use handlers::health_check;
use sqlx::postgres::PgPoolOptions;
use utils::appstate::AppState;

mod handlers;
mod models;
mod utils;

pub async fn app_router() -> Router {
    // initialize our database connection pool
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
        .unwrap();

    Router::new()
        .route("/", get(health_check::health_check))
        .route("/hello", get(handlers::hello::hello))
        .route("/user", post(handlers::user::create_user))
        .route("/user/:id", get(handlers::user::get_user_by_id))
        .route("/users", get(handlers::user::get_users))
        .route("/user/:id", delete(handlers::user::delete_user))
        .with_state(Arc::new(AppState { db: pool }))
}
