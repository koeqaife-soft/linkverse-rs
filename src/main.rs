use std::sync::Arc;

use crate::utils::state::AppState;
use axum::Router;
use dotenvy::dotenv;
use tracing::{error, info};
use tracing_subscriber;

mod database;
mod endpoints;
mod entities;
mod services;
mod utils;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let state = match AppState::create_from_env().await {
        Ok(state) => state,
        Err(err) => {
            error!("Failed to create AppState: {:?}", err);
            return;
        }
    };
    let shared_state = Arc::new(state);

    let router: Router<()> = endpoints::create_router().with_state(shared_state.clone());

    let listener = tokio::net::TcpListener::bind(shared_state.config.url.clone())
        .await
        .unwrap();
    info!("Listening on {:?}", shared_state.config.url);
    axum::serve(listener, router).await.unwrap();
}
