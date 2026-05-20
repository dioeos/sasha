use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use tokio::sync::broadcast;

mod exec;
mod client;

mod daemon;

mod niri;
mod stores;
mod events;

use crate::events::SashaEvent;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
     let subscriber = FmtSubscriber::builder()
         .with_max_level(Level::TRACE)
         .finish();
     tracing::subscriber::set_global_default(subscriber)
         .expect("Failed to set tracing subscriber!");

     info!("Starting sasha daemon...");

     let (tx, _) = broadcast::channel::<SashaEvent>(16);
     let niri_tx = tx.clone();
     // tokio::spawn(niri::read_niri_events(tx.clone()));
     tokio::spawn(async move {
         if let Err(err) = exec::read_niri_events(niri_tx).await {
             tracing::error!("Niri event task stopped: {err}");
         }
     });

     client::accept_sasha_clients(tx).await?;

     let sasha_daemon = daemon::Daemon::new();
     sasha_daemon.run().await?;

    Ok(())
}
