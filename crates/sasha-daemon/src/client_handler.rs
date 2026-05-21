use std::fs;
use tokio::{io::{BufWriter, AsyncWriteExt}, net::{UnixListener, UnixStream}, sync::broadcast};

use crate::events::SashaEvent;

pub struct ClientHandler {
    broadcaster: broadcast::Sender<SashaEvent>,
}

impl ClientHandler {
    pub fn new(tx: broadcast::Sender<SashaEvent>) -> Self {
        Self {
            broadcaster: tx
        }
    }

    pub async fn run(self) -> anyhow::Result<()> {
        let runtime_dir = std::env::var("XDG_RUNTIME_DIR").expect("XDG_RUNTIME_DIR is not set");
        let socket_path = format!("{runtime_dir}/sasha.sock");
        fs::remove_file(&socket_path);
        let listener = UnixListener::bind(&socket_path)?;

        loop {
            let (stream, addr) = listener.accept().await?;
            let rx = self.broadcaster.subscribe();
            tokio::spawn(self.handle_client(stream, rx));
        }
    }

    async fn handle_client(&self, stream: UnixStream, mut rx: broadcast::Receiver<SashaEvent>) -> anyhow::Result<()> {
        let mut writer = BufWriter::new(stream);

        loop {
            let event = rx.recv().await?;
            let json = serde_json::to_string(&event)?;
            writer.write_all(json.as_bytes()).await?;
            writer.write_all(b"\n").await?;
            writer.flush().await?;
        }

    }
}
