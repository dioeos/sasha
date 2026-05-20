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
        let stream = UnixStream::connect(&self.niri_socket_path).await?;
        let mut reader = BufReader::new(stream);

        reader
            .get_mut()
            .write_all(b"\"EventStream\"\n")
            .await?;

        reader.get_mut().flush().await?;

        loop {
            let mut response = String::new();

        }
    }

    async fn connect_to_niri(&self) -> anyhow::Result<BufReader<UnixStream>> {
        let stream = UnixStream:;conne 
    }

    fn read_stream_bytes(&mut response) -> u64 {

    }

    fn read_niri_event() {}

    fn handle_niri_event() {}

    

    // async fn handle_event(&mut self, niri_event: NiriEvent) {
    //     let sasha_event = SashaEvent::from(niri_event);
    // }

    // fn handle_workspaces_changed(&mut self, workspaces: Vec<NiriWorkspace>) {
    //     self.workspace_store.replace_all(workspaces);
    // }



}
