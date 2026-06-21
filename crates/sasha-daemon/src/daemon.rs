use std::sync::Arc;
use tracing::{span, Level, info, error, debug};
use tokio::sync::broadcast;
use anyhow::{Context, Result};
use crate::client_handler::ClientHandler;
use crate::command_listener::CommandListener;
use crate::stores::{WindowStore, WorkspaceStore, MarkStore};
use crate::niri::{NiriListener, NiriConnector};
use crate::events::{SashaEvent};

pub struct Daemon {
    niri_listener: NiriListener,
    command_listener: CommandListener,
    client_handler: ClientHandler,
}

impl Daemon {
    pub fn new() -> Result<Self> {
        let daemon_span = span!(Level::INFO, "[DAEMON]::new()");
        let _guard = daemon_span.enter();

        let (tx, _) = broadcast::channel::<SashaEvent>(16);

        let window_store = WindowStore::new();
        let workspace_store = WorkspaceStore::new();
        let mark_store = MarkStore::new();

        let niri_connector_arc = Arc::new(
            NiriConnector::new_from_env()
                .context("Failed to initialize Niri connector")?
        );

        debug!("Loaded necessary dependencies to create daemon");

        Ok(Self {
            niri_listener: NiriListener::new(
                window_store,
                workspace_store,
                niri_connector_arc.clone(),
                tx.clone()
            ),
            command_listener: CommandListener::new(
                mark_store,
                niri_connector_arc
            ),
            client_handler: ClientHandler::new(
                tx
            ),
        })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        let daemon_run_span = span!(Level::INFO, "[DAEMON]::run()");
        let _guard = daemon_run_span.enter();

        info!("Starting daemon...");
        debug!("Preparing to use tokio to spawn niri event listener async task");

        tokio::spawn(async move {
            if let Err(err) = self.niri_listener.run().await {
                error!("Niri event task stopped: {err}");
            }
        });

        tokio::spawn(async move {
            if let Err(err) = self.command_listener.run().await {
                error!("Could not handle commands");
            }
        });

        debug!("Preparing to run client handler for daemon");
        info!("Starting client handler for daemon...");

        //begins listening for client connections
        self.client_handler.run().await?;

        Ok(())
    }
}
