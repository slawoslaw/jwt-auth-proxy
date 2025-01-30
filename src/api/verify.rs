use crate::errors::RequestError;
use crate::jwt::{verify_token, Claims};
use crate::AppState;
use axum::extract::State;
use axum::{extract::rejection::JsonRejection, response::Json};
use jsonwebtoken::errors::{Error as JwtError, ErrorKind};

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
            let mut request_error = RequestError::InvalidToken("Problem with jwt".to_string());

            if let Some(jwt_err) = e.downcast_ref::<JwtError>() {
                match jwt_err.kind() {
                    ErrorKind::ExpiredSignature => {
                        request_error = RequestError::ExpiredToken();
                    }
                    _ => {}
                }
            }

            return Err(request_error);
        }
    };

    Ok(Json(VerifyResponse {
        status: String::from("ok"),
        token: claims,
    }))
}
