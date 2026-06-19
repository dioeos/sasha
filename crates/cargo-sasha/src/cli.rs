use tokio::{io::{AsyncWriteExt, BufWriter}, net::UnixStream};

use crate::command::{RequestPattern};


pub fn cargo_args() -> Vec<String> {
    let mut raw_args: Vec<String> = std::env::args().collect();
    
    if raw_args.get(1).map(String::as_str) == Some("sasha") {
        raw_args.remove(1);
    }
    raw_args
}

pub async fn execute_request(cmd: RequestPattern) -> anyhow::Result<()> {
    let json_payload = serde_json::to_string(&cmd)?;

    eprintln!("CLI reached execute_request");
    eprintln!("json payload: {json_payload}");

    let runtime_dir = std::env::var("XDG_RUNTIME_DIR")
        .expect("XDG_RUNTIME_DIR is not set");

    let socket_path = format!("{runtime_dir}/sasha-commands.sock");

    eprintln!("connecting to: {socket_path}");

    let stream = UnixStream::connect(&socket_path).await?;

    eprintln!("Connected");

    let mut writer = BufWriter::new(stream);

    writer.write_all(json_payload.as_bytes()).await?;
    writer.write_all(b"\n").await?;
    writer.flush().await?;

    eprintln!("Sent command");

    Ok(())
}
