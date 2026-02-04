use server::net::protocol::{parse_client_message, server_message_text};
use shared::protocol::{ClientMessage, ServerMessage};

#[test]
fn parse_plain_ping() {
    let parsed = parse_client_message("ping");
    assert!(matches!(parsed, Some(ClientMessage::Ping)));
}

#[test]
fn parse_json_ping() {
    let json = serde_json::to_string(&ClientMessage::Ping).expect("serialize ping");
    let parsed = parse_client_message(&json);
    assert!(matches!(parsed, Some(ClientMessage::Ping)));
}

#[test]
fn server_message_serializes() {
    let json = server_message_text(&ServerMessage::Pong).expect("serialize pong");
    assert!(json.contains("Pong"));
}
