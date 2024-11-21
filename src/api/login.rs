use axum::{extract::rejection::JsonRejection, response::Json};

use crate::errors::RequestError;
use crate::jwt::generate_token;
use log::error;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Debug)]
pub struct LoginPayload {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}

pub async fn handler(
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
        let jwt = match generate_token(&private_key_path, &payload.username) {
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
