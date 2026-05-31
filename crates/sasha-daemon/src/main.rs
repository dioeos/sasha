use tracing::{span, Level};

mod daemon;
mod client_handler;
mod logger;

mod niri;
mod stores;
mod events;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let _worker_guard = logger::init_logger().expect("Failed to setup application logger");

    let main_spain = span!(Level::INFO, "[MAIN]");
    let _guard = main_spain.enter();

     let sasha_daemon = daemon::Daemon::new();
     sasha_daemon.run().await?;

    Ok(())
}
