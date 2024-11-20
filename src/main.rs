mod claims;
mod jwt;

use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Json, Response},
    routing::post,
    Router,
};
use claims::Claims;
use dotenv::dotenv;
use env_logger;
use jwt::{generate_jwt, verify_jwt};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::{env, fmt};

#[derive(Deserialize, Debug)]
struct LoginPayload {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct ErrorMessage {
    details: String,
}

impl ErrorMessage {
    fn new(details: String) -> Self {
        Self { details }
    }

    fn new_from_str(details: &str) -> Self {
        Self::new(String::from(details))
    }
}

#[derive(Debug)]
enum RequestError {
    SomethingWentWrong(),
    InvalidCredentials(),
    InvalidRequest(String),
}

impl RequestError {
    fn status_code(&self) -> StatusCode {
        match self {
            RequestError::SomethingWentWrong() => StatusCode::INTERNAL_SERVER_ERROR,
            RequestError::InvalidCredentials() => StatusCode::UNAUTHORIZED,
            RequestError::InvalidRequest(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestError::SomethingWentWrong() => write!(f, "Something went wrong"),
            RequestError::InvalidCredentials() => write!(f, "Invalid credentials"),
            RequestError::InvalidRequest(msg) => write!(f, "{}", msg),
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

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

#[derive(Deserialize, Debug)]
struct VerifyPayload {
    token: String,
}

#[derive(Serialize)]
struct VerifyResponse {
    status: String,
    token: Claims,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let app = Router::new()
        .route("/login", post(login))
        .route("/verify", post(verify))
        .fallback(handler_404);
    let addr: String = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "0.0.0.0:3000".to_string());
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!(
        "Server running on http://{}/",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}

async fn login(
    payload: Result<Json<LoginPayload>, JsonRejection>,
) -> Result<Json<LoginResponse>, RequestError> {
    let private_key_path = env::var("PRIVATE_KEY_PATH").map_err(|_| {
        error!("Missing PRIVATE_KEY_PATH env");
        RequestError::SomethingWentWrong()
    })?;

    let payload = payload.map_err(|_| {
        RequestError::InvalidRequest("Invalid JSON format or missing fields".to_string())
    })?;

    if payload.username.is_empty() || payload.password.is_empty() {
        return Err(RequestError::InvalidRequest(
            "Username or password cannot be empty".to_string(),
        ));
    }

    let auth_username = env::var("AUTH_USERNAME").map_err(|_| {
        error!("Missing AUTH_USERNAME env");
        RequestError::SomethingWentWrong()
    })?;

    let auth_password = env::var("AUTH_PASSWORD").map_err(|_| {
        error!("Missing AUTH_PASSWORD env");
        RequestError::SomethingWentWrong()
    })?;

    if payload.username == auth_username && payload.password == auth_password {
        let jwt = match generate_jwt(&private_key_path, &payload.username) {
            Ok(token) => token,
            Err(e) => {
                error!("{}", e);
                return Err(RequestError::InvalidRequest("Problem with jwt".to_string()));
            }
        };

        Ok(Json(LoginResponse { token: jwt }))
    } else {
        Err(RequestError::InvalidCredentials())
    }
}

async fn verify(
    payload: Result<Json<VerifyPayload>, JsonRejection>,
) -> Result<Json<VerifyResponse>, RequestError> {
    let public_key_path = env::var("PUBLIC_KEY_PATH").map_err(|_| {
        error!("Missing PUBLIC_KEY_PATH env");
        RequestError::SomethingWentWrong()
    })?;

    let payload = payload.map_err(|_| {
        RequestError::InvalidRequest("Invalid JSON format or missing fields".to_string())
    })?;

    if payload.token.is_empty() {
        return Err(RequestError::InvalidRequest(
            "Token cannot be empty".to_string(),
        ));
    }

    let claims = match verify_jwt(&public_key_path, &payload.token) {
        Ok(claims) => claims,
        Err(e) => {
            error!("{}", e);
            return Err(RequestError::InvalidRequest("Problem with jwt".to_string()));
        }
    };

    Ok(Json(VerifyResponse {
        status: String::from("ok"),
        token: claims,
    }))
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(ErrorMessage::new_from_str("Path does not exists")),
    )
}
