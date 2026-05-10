use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufReader, BufWriter};
use tokio::sync::broadcast;
use tokio::net::UnixStream;
use tracing::info;

use crate::events::SashaEvent;

pub async fn read_niri_events(tx: broadcast::Sender<SashaEvent>) -> anyhow::Result<()> {

    info!("Connecting to Niri event stream...");
    let niri_socket_path = std::env::var("NIRI_SOCKET").expect("NIRI_SOCKET is not set");
    let stream = UnixStream::connect(niri_socket_path).await?;
    let mut reader =BufReader::new(stream);

    // let (reader_stream, writer_stream) = stream.into_split();
    //
    // let mut writer = BufWriter::new(writer_stream);
    // let mut reader = BufReader::new(reader_stream);
    info!("Subscribing to niri event stream...");

    reader
        .get_mut()
        .write_all(b"\"EventStream\"\n")
        .await?;

    reader.get_mut().flush().await?;

    loop {
        let mut response = String::new();
        let bytes_read = reader.read_line(&mut response).await?;

        if bytes_read == 0 {
            info!("Niri event stream closed.");
            break;
        }
        info!("Niri event: {response}");
    }
    Ok(())
}
