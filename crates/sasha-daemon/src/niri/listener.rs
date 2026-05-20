use tokio::sync::broadcast::{Sender};
use tokio::net::UnixStream;
use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufReader, BufWriter};

use crate::events::{SashaEvent};
use crate::stores::{WindowStore, WorkspaceStore};

use super::models::{NiriEvent, NiriWorkspace, NiriWindow};


pub struct NiriListener {
    // tx: broadcast::Sender<SashaEvent>,
    window_store: WindowStore,
    workspace_store: WorkspaceStore,
    niri_socket_path: String,
}

impl NiriListener {

    pub fn new(ww_store: WindowStore, ws_store: WorkspaceStore, niri_socket_path: String) -> Self {
        Self {
            window_store: ww_store,
            workspace_store: ws_store,
            niri_socket_path: niri_socket_path
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
        match event {
            NiriEvent::WorkspacesChanged { workspaces } => {

            }

            NiriEvent::WindowsChanged { windows } => {

            }

            NiriEvent::WindowFocusChanged { id } => {

            }

            NiriEvent::WorkspaceActivated { id } => {

            }

            NiriEvent::WindowOpenedOrChanged { window } => {

            }

        }
    }


    // fn handle_workspaces_changed(&mut self, workspaces: Vec<NiriWorkspace>) {
    //     self.workspace_store.replace_all(workspaces);
    // }



}
