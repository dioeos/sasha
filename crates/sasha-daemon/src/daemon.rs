use tracing::{span, Level, info, error};

use tokio::sync::broadcast;

use crate::client_handler::ClientHandler;
use crate::stores::{WindowStore, WorkspaceStore};
use crate::niri::{NiriListener};
use crate::events::{SashaEvent};

pub struct Daemon {
    tx: broadcast::Sender<SashaEvent>,
    niri_listener: NiriListener,
    client_handler: ClientHandler
}

impl Daemon {
    pub fn new() -> Self {
        let daemon_span = span!(Level::INFO, "[DAEMON]::new()");
        let _guard = daemon_span.enter();

        let (tx, _) = broadcast::channel::<SashaEvent>(16);

        let window_store = WindowStore::new();
        let workspace_store = WorkspaceStore::new();

        let niri_socket_path: String = std::env::var("NIRI_SOCKET")
            .expect("NIRI_SOCKET is not set");

        info!("Loaded necessary dependencies in daemon");

        Self {
            niri_listener: NiriListener::new(
                window_store,
                workspace_store,
                niri_socket_path,
                tx.clone()
            ),
            client_handler: ClientHandler::new(
                tx.clone()
            ),
            tx,
        }
    }

    pub async fn run(self) -> anyhow::Result<()> {
        let daemon_run_span = span!(Level::INFO, "[DAEMON]::run()");
        let _guard = daemon_run_span.enter();

        //spawns tokio niri event listener task
        info!("Using tokio to spawn niri event listener async task");

        tokio::spawn(async move {
            if let Err(err) = self.niri_listener.run().await {
                error!("Niri event task stopped: {err}");
            }
        });
        //begins listening for client connections
        self.client_handler.run().await?;
        Ok(())
    }
}
