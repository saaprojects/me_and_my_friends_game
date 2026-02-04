use crate::prelude::*;

pub mod http;
pub mod protocol;
pub mod ws;

pub fn router() -> Router {
    Router::new()
        .route("/health", get(http::health))
        .route("/ws", get(ws::ws_handler))
}
