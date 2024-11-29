use std::fs::{self};

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub jti: String,
    pub exp: usize,
}

impl Claims {
    pub fn new(sub: &str, duration_hours: i64) -> Self {
        let expiration = Utc::now() + Duration::hours(duration_hours);

        Claims {
            sub: sub.to_owned(),
            jti: Uuid::new_v4().to_string(),
            exp: expiration.timestamp() as usize,
        }
    }
}

pub fn generate_token(
    private_key_path: &str,
    user: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let private_key_bytes = fs::read(private_key_path)?;
    let private_key_slice: &[u8] = &private_key_bytes;
    let claims = Claims::new(user, 1); // 1-hour expiration
    let header = Header::new(Algorithm::ES256);
    let key = EncodingKey::from_ec_pem(&private_key_slice)?;
    let token = encode(&header, &claims, &key)?;

    Ok(token)
}

pub fn verify_token(
    public_key_path: &str,
    token: &str,
) -> Result<Claims, Box<dyn std::error::Error>> {
    let public_key_bytes = fs::read(public_key_path)?;
    let public_key_slice: &[u8] = &public_key_bytes;
    let key = DecodingKey::from_ec_pem(&public_key_slice)?;
    let validation = Validation::new(Algorithm::ES256);
    let decoded = decode::<Claims>(token, &key, &validation)?;

    Ok(decoded.claims)
}
