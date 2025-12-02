use crate::utils::state::AppState;
use axum::Router;
use dotenvy::dotenv;
use tracing::{error, info};
use tracing_subscriber;

mod endpoints;
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

    let router: Router<()> = endpoints::create_router().with_state(state.clone());

    let listener = tokio::net::TcpListener::bind(state.config.url.clone())
        .await
        .unwrap();
    axum::serve(listener, router).await.unwrap();
}
