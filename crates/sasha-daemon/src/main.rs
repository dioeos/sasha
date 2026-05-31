use tracing::{Level};
use tracing_subscriber::FmtSubscriber;

mod daemon;
mod client_handler;
mod logger;

mod niri;
mod stores;
mod events;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
     // let subscriber = FmtSubscriber::builder()
     //     .with_max_level(Level::TRACE)
     //     .finish();
     // tracing::subscriber::set_global_default(subscriber)
     //     .expect("Failed to set tracing subscriber!");


     // let _logger_guard = logger::init_logger()
     //     .expect("Failed to setup application logger");
     logger::init_logger().expect("Failed to setup application logger");

     let sasha_daemon = daemon::Daemon::new();
     sasha_daemon.run().await?;

    Ok(())
}
