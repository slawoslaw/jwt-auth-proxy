use crate::claims::Claims;
use crate::errors::RequestError;
use crate::jwt::verify_token;
use axum::{extract::rejection::JsonRejection, response::Json};

use log::error;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Debug)]
pub struct VerifyPayload {
    token: String,
}

#[derive(Serialize)]
pub struct VerifyResponse {
    status: String,
    token: Claims,
}

pub async fn handler(
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

    let claims = match verify_token(&public_key_path, &payload.token) {
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
