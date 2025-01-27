mod api;
mod errors;
mod jwt;
mod utils;

use axum::{routing::post, Router};

use dotenv::dotenv;
use env_logger;

use log::info;
use std::{env, sync::Arc};
use utils::{
    env::load_env_int,
    key::{LocalFileKeyLoader, PrivateKeyProvider, PublicKeyProvider},
};

#[derive(Clone)]
pub struct AppState {
    pub private_key_provider: Arc<dyn PrivateKeyProvider + Send + Sync>,
    pub public_key_provider: Arc<dyn PublicKeyProvider + Send + Sync>,
    pub token_duration: i64,
}

pub fn create_app_state_from_env() -> Result<AppState, Box<dyn std::error::Error>> {
    let private_key_path = std::env::var("PRIVATE_KEY_PATH")?;
    let public_key_path = std::env::var("PUBLIC_KEY_PATH")?;

    let private_loader = LocalFileKeyLoader {
        key_path: private_key_path,
    };
    let public_loader = LocalFileKeyLoader {
        key_path: public_key_path,
    };

    const DEFAULT_TOKEN_DURATION: i64 = 20;
    let token_duration = load_env_int("TOKEN_DURATION_MIN", DEFAULT_TOKEN_DURATION);

    Ok(AppState {
        private_key_provider: Arc::new(private_loader),
        public_key_provider: Arc::new(public_loader),
        token_duration,
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();
    let app_state = create_app_state_from_env()?;
    let app = Router::new()
        .route("/login", post(api::login::handler))
        .route("/verify", post(api::verify::handler))
        .fallback(api::not_found::handler)
        .with_state(app_state);
    let addr: String = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "0.0.0.0:3000".to_string());
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    info!(
        "Server running on http://{}/",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();
    Ok(())
}
