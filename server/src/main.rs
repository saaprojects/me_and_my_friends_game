#[tokio::main]
async fn main() {
    server::app::run().await;
}
