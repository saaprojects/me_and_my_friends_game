use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::core::{schedule::StartupTimer, AppConfig};
use crate::net;
use crate::prelude::*;

pub async fn run() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "server=info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let boot = StartupTimer::start();
    let config = AppConfig::from_env();
    let app = net::router();

    info!("server listening on {}", config.addr);
    boot.finish();
    axum::Server::bind(&config.addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
