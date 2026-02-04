use shared::protocol::{ClientMessage, Health, ServerMessage};

#[test]
fn health_serializes() {
    let payload = Health {
        status: "ok".to_string(),
    };
    let json = serde_json::to_string(&payload).expect("serialize health");
    assert!(json.contains("ok"));
}

#[test]
fn client_message_roundtrip() {
    let msg = ClientMessage::Ping;
    let json = serde_json::to_string(&msg).expect("serialize client message");
    let decoded: ClientMessage =
        serde_json::from_str(&json).expect("deserialize client message");
    assert!(matches!(decoded, ClientMessage::Ping));
}

#[test]
fn server_message_roundtrip() {
    let msg = ServerMessage::Pong;
    let json = serde_json::to_string(&msg).expect("serialize server message");
    let decoded: ServerMessage =
        serde_json::from_str(&json).expect("deserialize server message");
    assert!(matches!(decoded, ServerMessage::Pong));
}
