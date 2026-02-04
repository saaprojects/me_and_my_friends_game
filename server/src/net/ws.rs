use crate::prelude::*;
use crate::services;

pub async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        match msg {
            Ok(message) => match services::ws::handle_message(message) {
                services::ws::SocketAction::Send(response) => {
                    let _ = socket.send(response).await;
                }
                services::ws::SocketAction::Close => {
                    break;
                }
                services::ws::SocketAction::Ignore => {}
            },
            Err(err) => {
                error!("websocket error: {}", err);
                break;
            }
        }
    }
}
