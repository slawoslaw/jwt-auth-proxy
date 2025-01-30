use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};

use serde::Serialize;
use std::fmt;

#[derive(Serialize)]
pub struct ErrorMessage {
    details: String,
}

impl ErrorMessage {
    pub fn new(details: String) -> Self {
        Self { details }
    }

    pub fn new_from_str(details: &str) -> Self {
        Self::new(String::from(details))
    }
}

#[derive(Debug)]
pub enum RequestError {
    SomethingWentWrong(),
    InvalidCredentials(),
    InvalidToken(String),
    ExpiredToken(),
    InvalidRequest(String),
    InternalError(String)
}

impl RequestError {
    fn status_code(&self) -> StatusCode {
        match self {
            RequestError::SomethingWentWrong() => StatusCode::INTERNAL_SERVER_ERROR,
            RequestError::InvalidCredentials() => StatusCode::UNAUTHORIZED,
            RequestError::InvalidToken(_) => StatusCode::UNAUTHORIZED,
            RequestError::ExpiredToken() => StatusCode::UNAUTHORIZED,
            RequestError::InvalidRequest(_) => StatusCode::BAD_REQUEST,
            RequestError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestError::SomethingWentWrong() => write!(f, "Something went wrong"),
            RequestError::InvalidCredentials() => write!(f, "Invalid credentials"),
            RequestError::InvalidToken(msg) => write!(f, "{}", msg),
            RequestError::ExpiredToken() => write!(f, "Expired token"),
            RequestError::InvalidRequest(msg) => write!(f, "{}", msg),
            RequestError::InternalError(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for RequestError {}

impl IntoResponse for RequestError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let err_msg = ErrorMessage::new(self.to_string());
        (status, Json(err_msg)).into_response()
    }
}
