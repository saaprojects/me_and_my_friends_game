use crate::prelude::*;

pub fn parse_client_message(text: &str) -> Option<ClientMessage> {
    if text.eq_ignore_ascii_case("ping") {
        return Some(ClientMessage::Ping);
    }
    serde_json::from_str::<ClientMessage>(text).ok()
}

pub fn server_message_text(message: &ServerMessage) -> Option<String> {
    serde_json::to_string(message).ok()
}
