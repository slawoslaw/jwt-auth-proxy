use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use chrono::Duration;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::{
    key::{PrivateKeyProvider, PublicKeyProvider},
    time::{SystemTimeProvider, TimeProvider},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub jti: String,
    pub exp: usize,
}

impl Claims {
    pub fn new<T: TimeProvider>(time_provider: &T, sub: &str, duration_minutes: i64) -> Self {
        let expiration = time_provider.now() + Duration::hours(duration_minutes);

        Claims {
            sub: sub.to_owned(),
            jti: Uuid::new_v4().to_string(),
            exp: expiration.timestamp() as usize,
        }
    }
}

pub fn generate_token(
    private_key_provider: &dyn PrivateKeyProvider,
    user: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let private_key = private_key_provider.load()?;
    let claims = Claims::new(&SystemTimeProvider, user, 30);
    let header = Header::new(Algorithm::ES256);
    let key = EncodingKey::from_ec_pem(&private_key)?;
    let token = encode(&header, &claims, &key)?;

    Ok(token)
}

pub fn verify_token(
    public_key_provider: &dyn PublicKeyProvider,
    token: &str,
) -> Result<Claims, Box<dyn std::error::Error>> {
    let public_key = public_key_provider.load()?;
    let key = DecodingKey::from_ec_pem(&public_key)?;
    let validation = Validation::new(Algorithm::ES256);
    let decoded = decode::<Claims>(token, &key, &validation)?;

    Ok(decoded.claims)
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use regex::Regex;

    use super::*;

    pub struct MockTimeProvider {
        mock_time: DateTime<Utc>,
    }

    impl MockTimeProvider {
        pub fn new(mock_time: DateTime<Utc>) -> Self {
            Self { mock_time }
        }
    }

    impl TimeProvider for MockTimeProvider {
        fn now(&self) -> DateTime<Utc> {
            self.mock_time
        }
    }

    #[test]
    fn test_claims() {
        let mock_time = Utc::now();
        let mock_provider = MockTimeProvider::new(mock_time);
        let test_user = "test@user.com";
        let uuid_regex =
            Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$").unwrap();
        let claims = Claims::new(&mock_provider, test_user, 2);

        assert_eq!(claims.sub, test_user);
        assert!(uuid_regex.is_match(&claims.jti));
        assert_eq!(
            claims.exp,
            (mock_time + Duration::hours(2)).timestamp() as usize
        );
    }
}
