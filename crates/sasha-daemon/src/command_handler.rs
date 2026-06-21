use std::sync::Arc;
use anyhow::{Context, Result, anyhow};
use serde_json::json;
use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::{UnixStream}};
use tracing::{span, Level, info, debug, warn};

use crate::niri::NiriConnector;
use crate::stores::MarkStore;

use crate::events::RequestEvent;

pub struct CommandHandler {
    mark_store: MarkStore,
    niri_connector: Arc<NiriConnector>
}

impl CommandHandler {
    pub fn new(mk_store: MarkStore, connector: Arc<NiriConnector>) -> Self {
        let command_handler_span = span!(
            Level::INFO,
            "[COMMAND_HANDLER]::new()"
        );
        let _guard = command_handler_span.enter();

        Self {
            mark_store: mk_store,
            niri_connector: connector
        }
    }

    pub async fn handle_request_event(
        &mut self,
        event: RequestEvent
    ) -> Result<()> {
        match event {
            RequestEvent::MarkWindow { slot } => {
                let focused_id = match self
                    .get_current_focused_window_id()
                    .await {
                        Ok(id) => id,
                        Err(_) => {
                            warn!("Failed to get current focused window id");
                            return Ok(());
                        }
                    };

                let Some(window_id) = focused_id else {
                    warn!("No focused window to mark");
                    return Ok(());
                };
                self.mark_store.map.insert(slot, window_id);
            }

            RequestEvent::FocusWindow { slot } => {
                self.focus_requested_mark_window(slot).await?;
            }
        }
        Ok(())
    }

    async fn get_current_focused_window_id(&self) -> Result<Option<u64>> {
        let stream: UnixStream = self
            .niri_connector
            .connect()
            .await
            .context("Failed to create unix stream with Niri")?;

        let mut reader: BufReader<UnixStream> = BufReader::new(stream);

        reader
            .get_mut()
            .write_all(b"\"FocusedWindow\"\n")
            .await
            .context("Failed to send 'FocusedWindow' command to Niri")?;

        reader
            .get_mut()
            .flush()
            .await?;

        let mut response = String::new();
        reader
            .read_line(&mut response)
            .await?;

        let json: serde_json::Value = serde_json::from_str(&response)?;

        let id = json
            .get("Ok")
            .and_then(|ok| ok.get("FocusedWindow"))
            .and_then(|window| window.get("id"))
            .and_then(|id| id.as_u64());

        Ok(id)
    }

    async fn focus_requested_mark_window(&self, slot: u8) -> Result<()> {
        let window_id = self
            .mark_store.map
            .get(&slot)
            .ok_or_else(|| anyhow!("No window marked in slot {slot}"))?;

        let request = json!({
            "Action": {
                "FocusWindow": {
                    "id": window_id
                }
            }
        });

        let stream: UnixStream = self
            .niri_connector
            .connect()
            .await
            .context("Failed to create unix stream with Niri")?;

        let mut reader: BufReader<UnixStream> = BufReader::new(stream);
        
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
