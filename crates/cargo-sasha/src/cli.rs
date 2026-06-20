use tokio::{io::{AsyncWriteExt, BufWriter}, net::UnixStream};
use anyhow::{Result, Context};

use crate::command::{RequestPattern};


pub fn cargo_args() -> Vec<String> {
    let mut raw_args: Vec<String> = std::env::args().collect();
    
    if raw_args.get(1).map(String::as_str) == Some("sasha") {
        raw_args.remove(1);
    }
    raw_args
}

pub async fn send_request(cmd: RequestPattern) -> Result<()> {
    match cmd {
        RequestPattern::MarkWindow { slot: _ } => {
            let json_payload = serde_json::to_string(&cmd)
                .context("Failed to serialize MarkWindow request to JSON")?;

            let stream = connect_to_commands_socket().await?;
            let mut writer = BufWriter::new(stream);
            writer.write_all(json_payload.as_bytes()).await?;
            writer.write_all(b"\n").await?;
            writer.flush().await?;
        }
        RequestPattern::FocusWindow { slot: _ } => {
            eprintln!("Focus window pattern...");
            let json_payload = serde_json::to_string(&cmd)
                .context("Failed to serialize FocusWindow request to JSON")?;
            let stream = connect_to_commands_socket().await?;
            let mut writer = BufWriter::new(stream);


            eprintln!("Writing...");
            writer.write_all(json_payload.as_bytes()).await?;
            writer.write_all(b"\n").await?;
            writer.flush().await?;
            eprintln!("Wrote...");
        }
    }
    Ok(())
}

async fn connect_to_commands_socket() -> Result<UnixStream> {
    let runtime_dir = std::env::var("XDG_RUNTIME_DIR")
        .expect("XDG_RUNTIME_DIR is not set");
    let socket_path = format!("{runtime_dir}/sasha-commands.sock");
    Ok(UnixStream::connect(&socket_path).await?)
}
