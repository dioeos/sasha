use tokio::sync::broadcast;

use crate::stores::{WindowStore, WorkspaceStore};
use crate::niri::{NiriListener};
use crate::events::{SashaEvent};

pub struct Daemon {
    tx: broadcast::Sender<SashaEvent>,
    niri_listener: NiriListener,
}

impl Daemon {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel::<SashaEvent>(16);

        let window_store = WindowStore::new();
        let workspace_store = WorkspaceStore::new();

        let niri_socket_path: String = std::env::var("NIRI_SOCKET")
            .expect("NIRI_SOCKET is not set");

        Self {
            niri_listener: NiriListener::new(
                               window_store,
                               workspace_store,
                               niri_socket_path
                           ),
            tx,
        }
    }

    pub async fn run(self) -> anyhow::Result<()> {
        let broadcaster_copy = self.tx.clone();
        //spawns tokio niri event listener task
        tokio::spawn(async move {
            if let Err(err) = self.niri_listener.run(broadcaster_copy).await {
                tracing::error!("Niri event task stopped: {err}");
            }
        });
        //begins listening for client connections
        Ok(())
    }
}
