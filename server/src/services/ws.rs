use crate::net::protocol;
use crate::prelude::*;

pub enum SocketAction {
    Send(Message),
    Close,
    Ignore,
}

pub fn handle_message(message: Message) -> SocketAction {
    match message {
        Message::Text(text) => {
            if let Some(client_msg) = protocol::parse_client_message(&text) {
                match client_msg {
                    ClientMessage::Ping => {
                        if let Some(payload) = protocol::server_message_text(&ServerMessage::Pong) {
                            return SocketAction::Send(Message::Text(payload));
                        }
                        return SocketAction::Ignore;
                    }
                }
            }
            SocketAction::Send(Message::Text(text))
        }
        Message::Binary(bytes) => SocketAction::Send(Message::Binary(bytes)),
        Message::Close(_) => SocketAction::Close,
        _ => SocketAction::Ignore,
    }
}
