use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::utils::error::ErrorResponse;

#[derive(Deserialize)]
pub struct HelloParams {
    name: Option<String>,
}

#[derive(Serialize)]
pub struct HelloResponse {
    message: String,
}

pub async fn hello(Query(params): Query<HelloParams>) -> Response {
    if params.name.is_none() {
        let error = ErrorResponse {
            code: 400,
            message: "param name cannot be empty".to_string(),
        };
        (StatusCode::BAD_REQUEST, Json(error)).into_response()
    } else {
        let message = format!("Hello, {}!", params.name.unwrap());
        (StatusCode::OK, Json(HelloResponse { message })).into_response()
    }
}
