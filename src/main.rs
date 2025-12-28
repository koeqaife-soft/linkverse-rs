use std::{any, sync::Arc, time::Duration};

use crate::utils::state::AppState;
use axum::Router;
use dotenvy::dotenv;
use tower_http::{
    catch_panic::CatchPanicLayer,
    cors::{Any, CorsLayer},
};
use tracing::{error, info};
use tracing_subscriber;

use axum::http::Response;
use bytes::Bytes;
use http_body_util::Full;

mod database;
mod endpoints;
mod entities;
mod extractors;
mod services;
mod utils;

fn panic_handler(err: Box<dyn any::Any + Send + 'static>) -> Response<Full<Bytes>> {
    let msg = if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else {
        "Unknown panic".to_string()
    };
    error!("PANIC: {}", msg);

    let body = serde_json::json!({
        "success": false,
        "error": "INTERNAL_SERVER_ERROR",
    })
    .to_string();

    Response::builder()
        .status(500)
        .header("content-type", "application/json")
        .body(Full::from(body))
        .unwrap()
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::fmt()
        .with_writer(std::io::stderr)
        .init();

    let state = match AppState::create_from_env().await {
        Ok(state) => state,
        Err(err) => {
            error!("Failed to create AppState: {:?}", err);
            return;
        }
    };
    let shared_state = Arc::new(state);

    let v1_router: Router<()> = endpoints::create_router().with_state(shared_state.clone());
    let router = Router::new().nest("/v1", v1_router).layer(
        tower::ServiceBuilder::new()
            .layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods(Any)
                    .allow_headers(Any)
                    .max_age(Duration::from_secs(3600)),
            )
            .layer(CatchPanicLayer::custom(panic_handler)),
    );

    let listener = tokio::net::TcpListener::bind(shared_state.config.url.clone())
        .await
        .unwrap();
    info!("Listening on {:?}", shared_state.config.url);
    axum::serve(listener, router).await.unwrap();
}
