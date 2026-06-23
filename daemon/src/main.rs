//! IBus Buffalo Input Method Daemon main entrypoint.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ibus_buffalo::run_daemon().await
}
