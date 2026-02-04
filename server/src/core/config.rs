use std::net::SocketAddr;

#[derive(Clone, Copy, Debug)]
pub struct AppConfig {
    pub addr: SocketAddr,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let addr = std::env::var("SERVER_ADDR")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or_else(|| SocketAddr::from(([0, 0, 0, 0], 8000)));
        Self { addr }
    }
}
