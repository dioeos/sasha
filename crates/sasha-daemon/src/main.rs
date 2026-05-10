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
     tokio::spawn(niri::read_niri_events(tx.clone()));

     client::accept_sasha_clients(tx).await?;

    Ok(())
}
