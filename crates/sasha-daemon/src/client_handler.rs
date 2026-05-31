use std::fs;
use tokio::{io::{BufWriter, AsyncWriteExt}, net::{UnixListener, UnixStream}, sync::broadcast};

use tracing::{span, Level, info, error, debug, trace, warn};

use crate::events::SashaEvent;

pub struct ClientHandler {
    broadcaster: broadcast::Sender<SashaEvent>,
}

impl ClientHandler {
    pub fn new(tx: broadcast::Sender<SashaEvent>) -> Self {
        let client_h_span = span!(Level::INFO, "[CLIENT_HANDLER]::new()");
        let _guard = client_h_span.enter();

        Self {
            broadcaster: tx
        }
    }

    pub async fn run(self) -> anyhow::Result<()> {
        let client_run_span = span!(Level::INFO, "[CLIENT_HANDLER]::run()");
        let _guard = client_run_span.enter();

        let runtime_dir = std::env::var("XDG_RUNTIME_DIR").expect("XDG_RUNTIME_DIR is not set");
        let socket_path = format!("{runtime_dir}/sasha-events.sock");
        let _ = fs::remove_file(&socket_path);
        let listener = UnixListener::bind(&socket_path)?;

        info!("Entering client listening loop...");

        loop {
            let (stream, _) = listener.accept().await?;
            let rx = self.broadcaster.subscribe();
            tokio::spawn(async move {
                if let Err(err) = ClientHandler::handle_client(stream, rx).await {
                    warn!("Sasha client disconnected: {err}");
                }
            });
        }
    }

    async fn handle_client(stream: UnixStream, mut rx: broadcast::Receiver<SashaEvent>) -> anyhow::Result<()> {
        let handler_span = span!(Level::INFO, "[CLIENT_HANDLER]::handle_client()");
        let _guard = handler_span.enter();

        let mut writer = BufWriter::new(stream);

        loop {
            let event = rx.recv().await?;
            let json = serde_json::to_string(&event)?;

            debug!("JSON payload to broadcast to subscribers: {json}");

            writer.write_all(json.as_bytes()).await?;
            writer.write_all(b"\n").await?;
            writer.flush().await?;

            debug!("Successfully broadcasted JSON payload to subscribers");
        }
    }
}
