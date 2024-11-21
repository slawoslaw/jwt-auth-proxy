use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};

use crate::errors::ErrorMessage;

pub async fn handler() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(ErrorMessage::new_from_str("Path does not exists")),
    )
}
