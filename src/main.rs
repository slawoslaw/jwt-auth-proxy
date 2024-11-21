mod api;
mod claims;
mod errors;
mod jwt;

use axum::{routing::post, Router};

use dotenv::dotenv;
use env_logger;

use log::info;

use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let app = Router::new()
        .route("/login", post(api::login::handler))
        .route("/verify", post(api::verify::handler))
        .fallback(api::not_found::handler);
    let addr: String = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "0.0.0.0:3000".to_string());
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    info!(
        "Server running on http://{}/",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}
