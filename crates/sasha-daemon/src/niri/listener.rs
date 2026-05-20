use tokio::sync::broadcast;
use tokio::sync::broadcast::{Sender};
use tokio::net::UnixStream;
use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufReader, BufWriter};

use crate::events::sasha_event::SashaWorkspace;
use crate::events::{SashaEvent};
use crate::stores::{WindowStore, WorkspaceStore};

use super::models::{NiriEvent, NiriWorkspace, NiriWindow};


pub struct NiriListener {
    window_store: WindowStore,
    workspace_store: WorkspaceStore,
    niri_socket_path: String,
    broadcaster: broadcast::Sender<SashaEvent>
}

impl NiriListener {

    pub fn new(ww_store: WindowStore, ws_store: WorkspaceStore, niri_socket_path: String, tx: broadcast::Sender<SashaEvent>) -> Self {
        Self {
            window_store: ww_store,
            workspace_store: ws_store,
            niri_socket_path: niri_socket_path,
            broadcaster: tx
        }
    }

    pub async fn run(mut self, broadcaster: Sender::<SashaEvent>) -> anyhow::Result<()> {
        let mut reader = self.connect_to_niri().await?;

        loop {
            let mut response = String::new();
            let bytes_read = reader.read_line(&mut response).await?;

            if bytes_read == 0 {
                // info!("Niri event stream closed.");
                break;
            }

            let event: NiriEvent = self.read_niri_event(&response)?;
            self.handle_niri_event(event);
        }
        Ok(())
    }

    async fn connect_to_niri(&self) -> anyhow::Result<BufReader<UnixStream>> {
        let stream = UnixStream::connect(&self.niri_socket_path).await?;
        let mut reader = BufReader::new(stream);

        reader
            .get_mut()
            .write_all(b"\"EventStream\"\n")
            .await?;

        reader.get_mut().flush().await?;

        Ok(reader)
    }

    fn read_niri_event(&self, response: &str) -> anyhow::Result<NiriEvent> {
        match serde_json::from_str(response) {
            Ok(data) => Ok(data),
            Err(err) => Err(err.into()),
        }
    }

    fn handle_niri_event(&mut self, event: NiriEvent) {
        if let Some(sasha_event) = self.convert_niri_event(event) {
            self.broadcaster.send(sasha_event);
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

    fn convert_niri_event(&mut self, event: NiriEvent) -> Option<SashaEvent> {
        match event {
            NiriEvent::WorkspacesChanged { workspaces } => {
                Some(self.handle_workspaces_changed(workspaces))
            }

            NiriEvent::WindowsChanged { windows } => {
                //dirty

            }

            NiriEvent::WindowFocusChanged { id } => {
                //pure

            }

            NiriEvent::WorkspaceActivated { id } => {
                //pure

            }

            NiriEvent::WindowOpenedOrChanged { window } => {
                //pure

            }

        }
    }
}
