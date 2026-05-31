use tracing::{span, Level};

mod daemon;
mod client_handler;
mod logger;

mod niri;
mod stores;
mod events;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    logger::init_logger().expect("Failed to setup application logger");

    let main_spain = span!(Level::INFO, "[MAIN]");
    let _guard = main_spain.enter();

     // let _logger_guard = logger::init_logger()
     //     .expect("Failed to setup application logger");
    
     let sasha_daemon = daemon::Daemon::new();
     sasha_daemon.run().await?;

    Ok(())
}
