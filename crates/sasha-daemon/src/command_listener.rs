use std::{env::var_os, ffi::OsString, fs, io::ErrorKind, path::PathBuf, sync::Arc};
use serde_json::json;
use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::{UnixListener, UnixStream}};
use anyhow::{Context, anyhow, Result};
use tracing::{span, Level, info, debug, warn};

use crate::events::RequestEvent;
use crate::{command_handler::CommandHandler, niri::NiriConnector, stores::MarkStore};

pub struct CommandListener {
    mark_store: MarkStore,
    niri_connector: Arc<NiriConnector>,
}

impl CommandListener {
    pub fn new(
        mk_store: MarkStore,
        connector: Arc<NiriConnector>
    ) -> Self {

        let command_listener_span = span!(Level::INFO, "[COMMAND]::new()");
        let _guard = command_listener_span.enter();

        Self {
            mark_store: mk_store,
            niri_connector: connector
        }
    }

    pub async fn run(self) -> anyhow::Result<()> {
        let command_run_span = span!(Level::INFO, "[COMMAND]::run()");
        let _guard = command_run_span.enter();

        let xdg_os_string: OsString = var_os("XDG_RUNTIME_DIR")
            .ok_or_else(|| anyhow!("Cannot find XDG_RUNTIME_DIR var"))?;
        let xdg_runtime_path: PathBuf = PathBuf::from(xdg_os_string).join("sasha-commands.sock");

        let listener: UnixListener = Self::create_unix_listener(xdg_runtime_path)
            .context("Failed to create Sasha command listener")?;

        let mut handler = CommandHandler::new(
            self.mark_store,
            self.niri_connector
        );

        loop {
            let mut response = String::new();
            let (stream, _) = listener.accept().await?;
            debug!("Someone connected!");
            let mut reader = BufReader::new(stream);

            let _bytes_read = reader.read_line(&mut response).await?;

            let request_model: RequestEvent = Self::read_cli_request(&response)?;
            handler.handle_request_event(request_model).await?;
        }
    }

    fn create_unix_listener(socket_path: PathBuf) -> Result<UnixListener> {
        fs::remove_file(&socket_path)
            .or_else(|err| {
                if err.kind() == ErrorKind::NotFound {
                    Ok(())
                } else {
                    Err(err)
                }
            })
        .with_context(|| format!(
                "Failed to remove stale socket at {}",
                socket_path.display()
        ))?;

        let listener = UnixListener::bind(&socket_path)
            .with_context(|| format!(
                    "Failed to bind to socket at {}",
                    socket_path.display()
            ))?;
        Ok(listener)
    }

    fn read_cli_request(response: &str) -> Result<RequestEvent> {
        let request = serde_json::from_str(response)
            .context("Failed to parse CLI JSON request")?;

        Ok(request)
    }

}
