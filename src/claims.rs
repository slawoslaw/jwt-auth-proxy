use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

impl Claims {
    pub fn new(sub: &str, duration_hours: i64) -> Self {
        let expiration = Utc::now() + Duration::hours(duration_hours);
        Claims {
            sub: sub.to_owned(),
            exp: expiration.timestamp() as usize,
        }
    }
}
