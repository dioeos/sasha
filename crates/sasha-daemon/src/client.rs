use std::fs;
use tokio::sync::broadcast;
use tracing::info;
use tokio::net::UnixListener;
use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufReader, BufWriter};


use crate::events::SashaEvent;

pub async fn accept_sasha_clients(tx: broadcast::Sender<SashaEvent>) -> anyhow::Result<()> {
    info!("Attempting to accept clients on Sasha UNIX stream...");

    let runtime_dir = std::env::var("XDG_RUNTIME_DIR").expect("XDG_RUNTIME_DIR is not set");
    let socket_path = format!("{runtime_dir}/sasha.sock");

    fs::remove_file(&socket_path).expect("Could not clear existing sasha socket");

    let listener = UnixListener::bind(&socket_path)?;
    info!("Sasha established listener socket at {socket_path}");

    loop {
        let (stream, _) = listener.accept().await?;
        let rx = tx.subscribe();

        info!("Sasha client connected.");
        tokio::spawn(handle_client(stream, rx));
    }
}

async fn handle_client(stream: tokio::net::UnixStream, mut rx: broadcast::Receiver<SashaEvent>) -> anyhow::Result<()> {
    let mut writer = BufWriter::new(stream);

    loop {
        let event = rx.recv().await?;
        let json = serde_json::to_string(&event)?;
        writer.write_all(json.as_bytes()).await?;
        writer.write_all(b"\n").await?;
        writer.flush().await?;
    }
}
