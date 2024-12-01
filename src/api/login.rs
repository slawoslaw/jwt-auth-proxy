use axum::{
    extract::{rejection::JsonRejection, State},
    response::Json,
};

use crate::jwt::generate_token;
use crate::{errors::RequestError, AppState};
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
    State(state): State<AppState>,
    payload: Result<Json<LoginPayload>, JsonRejection>,
) -> Result<Json<LoginResponse>, RequestError> {
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
        let jwt = match generate_token(&*state.private_key_provider, &payload.username) {
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
