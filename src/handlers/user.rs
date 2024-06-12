use std::sync::Arc;

use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use serde::Deserialize;

use crate::models::user::User;
use crate::utils::appstate::AppState;
use crate::utils::error::ErrorResponse;

#[derive(Deserialize)]
pub struct CreateUserSchema {
    name: String,
    email: String,
}

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateUserSchema>,
) -> Response {
    let query_result = sqlx::query_as!(
        User,
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *",
        body.name,
        body.email,
    )
    .fetch_one(&state.db)
    .await;

    match query_result {
        Ok(user) => {
            return (StatusCode::CREATED, Json(user)).into_response();
        }
        Err(e) => {
            tracing::error!("Failed to create user: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    code: 500,
                    message: e
                        .to_string()
                        .contains("duplicate key value violates unique constraint")
                        .then(|| "email already exists".to_string())
                        .unwrap_or_else(|| "internal server error".to_string()),
                }),
            )
                .into_response();
        }
    }
}

pub async fn get_user_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Response {
    let query_result = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(&state.db)
        .await;

    match query_result {
        Ok(user) => {
            return (StatusCode::OK, Json(user)).into_response();
        }
        Err(e) => {
            tracing::error!("Failed to fetch user: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    code: 500,
                    message: "internal server error".to_string(),
                }),
            )
                .into_response();
        }
    }
}

pub async fn get_users(State(state): State<Arc<AppState>>) -> Response {
    let query_result = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&state.db)
        .await;

    match query_result {
        Ok(users) => {
            return (StatusCode::OK, Json(users)).into_response();
        }
        Err(e) => {
            tracing::error!("Failed to fetch users: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    code: 500,
                    message: "internal server error".to_string(),
                }),
            )
                .into_response();
        }
    }
}

pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Response {
    let query_result = sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(&state.db)
        .await;

    match query_result {
        Ok(_) => {
            return (StatusCode::NO_CONTENT).into_response();
        }
        Err(e) => {
            tracing::error!("Failed to delete user: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    code: 500,
                    message: "internal server error".to_string(),
                }),
            )
                .into_response();
        }
    }
}
