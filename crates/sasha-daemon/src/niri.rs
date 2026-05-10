use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufReader, BufWriter};
use tokio::sync::broadcast;
use tokio::net::UnixStream;
use tracing::info;

use serde::Deserialize;

use crate::events::SashaEvent;

#[derive(serde::Deserialize, Debug)]
enum NiriEvent {
    Ok(String),
    WorkspacesChanged {
        workspaces: Vec<NiriWorkspace>
    },
    WindowsChanged {
        windows: Vec<NiriWindow>
    },
    WindowFocusChanged {
        id: u64
    },
}

#[derive(serde::Deserialize, Debug)]
struct NiriWorkspace {
    id: u64,
    idx: u64,
    name: Option<String>,
    output: String,
    is_urgent: bool,
    is_active: bool,
    is_focused: bool,
    active_window_id: Option<u64>
}

#[derive(serde::Deserialize, Debug)]
struct NiriWindow {
    id: u64,
    title: String,
    app_id: String,
    pid: u64,
    workspace_id: Option<u64>,
    is_focused: bool,
    is_floating: bool,
    is_urgent: bool
}

pub async fn read_niri_events(tx: broadcast::Sender<SashaEvent>) -> anyhow::Result<()> {

    info!("Connecting to Niri event stream...");
    let niri_socket_path = std::env::var("NIRI_SOCKET").expect("NIRI_SOCKET is not set");
    let stream = UnixStream::connect(niri_socket_path).await?;
    let mut reader =BufReader::new(stream);

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
        //should be parsing niri response and converting into sasha event
        // let _ = tx.send(sashaEvent)
        // let data: NiriEvent = serde_json::from_str(&response)?;
        let data: NiriEvent = match serde_json::from_str(&response) {
            Ok(data) => data,
            Err(err) => {
                let value: serde_json::Value = serde_json::from_str(&response)?;

                if let Some(obj) = value.as_object() {
                    if let Some(event_name) = obj.keys().next() {
                        info!("Ignoring niri event: {event_name}");
                    }
                } else {
                    info!("Failed to parse niri event: {err}");
                    info!("Raw event was: {response}");
                }

                continue;
            }
        };
        match data {
            NiriEvent::Ok(status) => {
                if status != "Handled" {
                    info!("Niri could not handle event stream subscribe request from Sasha.");
                }
            }
            NiriEvent::WorkspacesChanged { workspaces } => {
                for workspace in workspaces {
                    if workspace.is_focused {
                        info!(
                            "Focused workspace {} on output {}",
                            workspace.id,
                            workspace.output
                        )
                    }
                    //convert to SashaEvent
                }
            }
            NiriEvent::WindowsChanged { windows } => {
                for window in windows {
                    if window.is_focused {
                        info!(
                            "Focused window {}: {} ({}) on {}",
                            window.id,
                            window.title,
                            window.app_id,
                            window.workspace_id.unwrap_or(0)
                        )
                    }
                    //convert to SashaEvent
                }
            }
            NiriEvent::WindowFocusChanged { id } => {
                info!("Window focus changed {}", id);
            }
        }
    }
    Ok(())
}
