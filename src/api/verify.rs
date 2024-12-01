use crate::errors::RequestError;
use crate::jwt::{verify_token, Claims};
use crate::AppState;
use axum::extract::State;
use axum::{extract::rejection::JsonRejection, response::Json};

use log::error;
use serde::{Deserialize, Serialize};

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
    State(state): State<AppState>,
    payload: Result<Json<VerifyPayload>, JsonRejection>,
) -> Result<Json<VerifyResponse>, RequestError> {
    let payload = payload.map_err(|_| {
        RequestError::InvalidRequest("Invalid JSON format or missing fields".to_string())
    })?;

    if payload.token.is_empty() {
        return Err(RequestError::InvalidRequest(
            "Token cannot be empty".to_string(),
        ));
    }

    let claims = match verify_token(&*state.public_key_provider, &payload.token) {
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
