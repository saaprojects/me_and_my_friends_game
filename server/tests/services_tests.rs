use server::services::{health, ws};
use server::services::ws::SocketAction;

#[test]
fn health_is_ok() {
    let payload = health::health();
    assert_eq!(payload.status, "ok");
}

#[test]
fn echoes_text() {
    let action = ws::handle_message(axum::extract::ws::Message::Text("hello".into()));
    match action {
        SocketAction::Send(axum::extract::ws::Message::Text(text)) => assert_eq!(text, "hello"),
        _ => panic!("expected echo text"),
    }
}

#[test]
fn responds_to_ping() {
    let action = ws::handle_message(axum::extract::ws::Message::Text("ping".into()));
    match action {
        SocketAction::Send(axum::extract::ws::Message::Text(text)) => {
            assert!(text.contains("Pong"));
        }
        _ => panic!("expected pong response"),
    }
}

#[test]
fn closes_on_close_message() {
    let action = ws::handle_message(axum::extract::ws::Message::Close(None));
    assert!(matches!(action, SocketAction::Close));
}
