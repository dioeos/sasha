use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use tokio::sync::broadcast;

mod niri;
mod events;
mod client;

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
         if let Err(err) = niri::read_niri_events(niri_tx).await {
             tracing::error!("Niri event task stopped: {err}");
         }
     });

     client::accept_sasha_clients(tx).await?;

    Ok(())
}
