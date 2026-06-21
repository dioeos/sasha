use std::sync::Arc;

use anyhow::{Result, Context};

use tokio::sync::broadcast;
use tokio::net::UnixStream;
use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufReader};

use tracing::{span, Level, info, debug, warn};

use crate::events::{SashaEvent, SashaWindow, SashaWorkspace};
use crate::stores::{WindowStore, WorkspaceStore};

use super::models::{NiriEvent, NiriWorkspace, NiriWindow};
use super::connector::{NiriConnector};


pub struct NiriListener {
    window_store: WindowStore,
    workspace_store: WorkspaceStore,
    niri_connector: Arc<NiriConnector>,
    broadcaster: broadcast::Sender<SashaEvent>
}

impl NiriListener {
    pub fn new(
        ww_store: WindowStore,
        ws_store: WorkspaceStore,
        connector: Arc<NiriConnector>,
        tx: broadcast::Sender<SashaEvent>
    ) -> Self {
        let listener_span = span!(Level::INFO, "[LISTENER]::new()");
        let _guard = listener_span.enter();

        Self {
            window_store: ww_store,
            workspace_store: ws_store,
            niri_connector: connector,
            broadcaster: tx
        }
    }

    pub async fn run(mut self) -> anyhow::Result<()> {
        let lister_run_span = span!(Level::INFO, "[LISTENER]::run()");
        let _guard = lister_run_span.enter();

        let stream: UnixStream = self.niri_connector
            .connect()
            .await
            .context("Failed to create unix stream with Niri")?;

        let mut reader: BufReader<UnixStream> = BufReader::new(stream);
        self.send_event_stream_handshake(&mut reader).await?;


        loop {
            let mut response = String::new();
            let bytes_read = reader.read_line(&mut response).await?;

            if bytes_read == 0 {
                warn!("0 bytes were read. The niri event stream closed");
                break;
            }

            debug!("Raw niri response from EventStream: {}", response);

            match self.read_niri_event(&response) {
                Ok(event) => {
                    self.handle_niri_event(event);
                }
                Err(err) => {
                    warn!("Ignoring unaccounted NiriEvent variant: {err}");
                }
            }
        }
        Ok(())
    }

    async fn send_event_stream_handshake(
        &self,
        reader: &mut BufReader<UnixStream>
    ) -> Result<()> {
        reader
            .get_mut()
            .write_all(b"\"EventStream\"\n")
            .await
            .context("Failed to send EventStream handshake to Niri")?;
        Ok(())
    }

    fn read_niri_event(&self, response: &str) -> anyhow::Result<NiriEvent> {
        match serde_json::from_str(response) {
            Ok(data) => {
                debug!("Serialized niri event data: {}", data);
                Ok(data)
            }
            Err(err) => Err(err.into()),
        }
    }

    fn handle_niri_event(&mut self, event: NiriEvent) {
        info!("Handling NiriEvent: {}", event);

        if let Some(sasha_event) = self.convert_niri_event(event) {
            match self.broadcaster.send(sasha_event) {
                Ok(count) => {
                    tracing::debug!("Broadcasted sasha event to {count} clients");
                }
                Err(err) => {
                    tracing::warn!("No active sasha clients: {err}");
                }
            }
        }
    }

    fn handle_workspaces_changed(&mut self, workspaces: Vec<NiriWorkspace>) -> SashaEvent {
        let mut sasha_workspaces: Vec<SashaWorkspace> = workspaces
            .iter()
            .map(|workspace| SashaWorkspace::from(workspace))
            .collect();
        sasha_workspaces.sort_by_key(|workspace| workspace.idx);

        self.workspace_store.replace_all(workspaces);

        SashaEvent::SashaWorkspacesChanged { sasha_workspaces }
    }

    fn handle_windows_changed(&mut self, windows: Vec<NiriWindow>) -> SashaEvent {
        let mut sasha_windows: Vec<SashaWindow> = windows
            .iter()
            .map(|window| SashaWindow::from(window))
            .collect();
        sasha_windows.sort_by_key(|window| window.id);

        self.window_store.replace_all(windows);

        SashaEvent::SashaWindowsChanged { sasha_windows }
    }

    fn handle_window_focus_changed(&self, id: Option<u64>) -> SashaEvent {
        match id {
            Some(id) => {
                let window_name = self.window_store
                    .get_window_name(&id)
                    .unwrap_or("Unknown");
                

                SashaEvent::SashaWindowFocusedChanged {
                    id: Some(id),
                    window_name: window_name.to_string()
                }
            }
            None => {
                SashaEvent::SashaWindowFocusedChanged {
                    id: None,
                    window_name: "None".to_string()
                }
            }
        }
    }

    fn handle_workspace_activated(&self, id: u64) -> Option<SashaEvent> {
        let workspace = self.workspace_store.map.get(&id);
        match workspace {
            Some(workspace) => {
                let s_workspace = SashaWorkspace::from(workspace);
                Some(SashaEvent::SashaWorkspaceActivated { sasha_workspace: s_workspace })
            }
            None => None
        }
    }

    fn handle_window_opened_or_changed(&mut self, window: NiriWindow) -> SashaEvent {
        let name_copy = window.title.clone();
        let id_copy = window.id.clone();

        self.window_store.map.insert(window.id, window);

        SashaEvent::SashaWindowOpenedOrChanged {
            id: id_copy,
            window_name: name_copy
        }
    }

    fn handle_ok_event(&self, msg: String) -> SashaEvent {
        SashaEvent::Ok { msg: msg }
    }

    fn convert_niri_event(&mut self, event: NiriEvent) -> Option<SashaEvent> {
        match event {
            NiriEvent::Ok ( msg ) => {
                Some(self.handle_ok_event(msg))
            }
            NiriEvent::WorkspacesChanged { workspaces } => {
                Some(self.handle_workspaces_changed(workspaces))
            }

            NiriEvent::WindowsChanged { windows } => {
                Some(self.handle_windows_changed(windows))
            }

            NiriEvent::WindowFocusChanged { id } => {
                Some(self.handle_window_focus_changed(id))
            }

            NiriEvent::WorkspaceActivated { id } => {
                self.handle_workspace_activated(id)
            }

            NiriEvent::WindowOpenedOrChanged { window } => {
                Some(self.handle_window_opened_or_changed(window))
            }
        }
    }
}
