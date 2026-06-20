use std::fs;
use serde_json::json;
use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::{UnixListener, UnixStream}};
use serde::Deserialize;
use anyhow::{anyhow};
use tracing::{span, Level, info, debug, warn};

use crate::stores::MarkStore;

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RequestEvent {
    MarkWindow {
        slot: u8,
    },
    FocusWindow {
        slot: u8
    }
}

pub struct CommandListener {
    mark_store: MarkStore
}

impl CommandListener {
    pub fn new(mk_store: MarkStore) -> Self {
        let command_listener_span = span!(Level::INFO, "[COMMAND]::new()");
        let _guard = command_listener_span.enter();

        Self {
            mark_store: mk_store
        }
    }

    pub async fn run(mut self) -> anyhow::Result<()> {
        let command_run_span = span!(Level::INFO, "[COMMAND]::run()");
        let _guard = command_run_span.enter();

        let runtime_dir = std::env::var("XDG_RUNTIME_DIR").expect("XDG_RUNTIME_DIR is not set");
        let socket_path = format!("{runtime_dir}/sasha-commands.sock");
        let _ = fs::remove_file(&socket_path);
        let listener = UnixListener::bind(&socket_path)?;

        loop {
            let mut response = String::new();
            let (stream, _) = listener.accept().await?;
            debug!("Someone connected!");
            let mut reader = BufReader::new(stream);

            let _bytes_read = reader.read_line(&mut response).await?;

            match self.read_cli_request(&response) {
                Ok(event) => {
                    debug!("READ EVENT!");
                    let _ = self.handle_request_event(event).await?;
                }
                Err(err) => {
                    debug!("Something went wrong")
                }
            }
        }
    }

    fn read_cli_request(&self, response: &str) -> anyhow::Result<RequestEvent> {
        debug!("Reading request");
        match serde_json::from_str(response) {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => Err(err.into())
        }
    }

    async fn handle_request_event(&mut self, event: RequestEvent) -> anyhow::Result<()> {
        match event {
            RequestEvent::MarkWindow { slot } => {
                //get current focused ID and insert SLOT : ID
                debug!("TRYING TO MARK");
                let id = self.get_current_focused_window_id().await?;

                if let Some(id) = id {
                    self.mark_store.map.insert(slot, id);
                    debug!("INSERTED WINDOW");
                } else {
                    warn!("No focused window to mark")
                }
                Ok(())
            }
            RequestEvent::FocusWindow { slot } => {
                self.focus_requested_mark_window(slot).await?;
                Ok(())
            }
        }
    }

    async fn get_current_focused_window_id(&self) -> anyhow::Result<Option<u64>> {
        let niri_socket_path: String = std::env::var("NIRI_SOCKET")
            .expect("NIRI_SOCKET is not set");

        let stream = UnixStream::connect(niri_socket_path).await?;
        let mut reader = BufReader::new(stream);

        reader
            .get_mut()
            .write_all(b"\"FocusedWindow\"\n")
            .await?;

        reader.get_mut().flush().await?;
        let mut response = String::new();
        reader.read_line(&mut response).await?;
        let json: serde_json::Value = serde_json::from_str(&response)?;

        let id = json
            .get("Ok")
            .and_then(|ok| ok.get("FocusedWindow"))
            .and_then(|window| window.get("id"))
            .and_then(|id| id.as_u64());

        Ok(id)
    }

    async fn focus_requested_mark_window(&self, slot: u8) -> anyhow::Result<()> {
        let window_id = self.mark_store.map
            .get(&slot)
            .ok_or_else(|| anyhow!("No window marked in slot {slot}"))?;

        let request = json!({
            "Action": {
                "FocusWindow": {
                    "id": window_id
                }
            }
        });

        let niri_socket_path: String = std::env::var("NIRI_SOCKET")
            .expect("NIRI_SOCKET is not set");

        let stream = UnixStream::connect(niri_socket_path).await?;
        let mut reader = BufReader::new(stream);

        let payload = serde_json::to_string(&request)?;

        reader
            .get_mut()
            .write_all(payload.as_bytes())
            .await?;

        reader
            .get_mut()
            .write_all(b"\n").await?;

        reader
            .get_mut()
            .flush()
            .await?;

        Ok(())
    }

}
