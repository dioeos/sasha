use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufReader, BufWriter};
use tokio::sync::broadcast;
use tokio::net::UnixStream;
use tracing::info;

use std::collections::HashMap;

use serde::Deserialize;

use crate::events::SashaEvent;

#[derive(serde::Deserialize, Debug)]
enum NiriEvent {
    WorkspacesChanged {
        workspaces: Vec<NiriWorkspace>
    },
    WindowsChanged {
        windows: Vec<NiriWindow>
    },
    WindowFocusChanged {
        id: Option<u64>
    },
    WindowOpenedOrChanged {
        window: NiriWindow
    },
    WorkspaceActivated {
        id: u64
    }
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

pub struct WindowStore {
    pub map: HashMap<u64, String>
}

impl WindowStore {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub fn get_window_name(&self, key: &u64) -> Option<&String> {
        self.map.get(key)
    }
}

pub async fn read_niri_events(tx: broadcast::Sender<SashaEvent>) -> anyhow::Result<()> {
    let mut window_store = WindowStore::new();

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
                info!("Failed to parse niri event: {err}");
                info!("Raw event was: {response}");
                continue;
            }
        };
        match data {
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
                    window_store.map.insert(
                        window.id,
                        window.title.clone()
                    );
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
                match id {
                    Some(id) => {
                        if let Some(name) = window_store.get_window_name(&id) {
                            info!("Window focus changed {} | {}", id, name);
                            let sevt = SashaEvent::SashaWindowFocusedChanged {
                                id: Some(id),
                                window_name: name.clone()
                            };
                            match tx.send(sevt) {
                                Ok(count) => info!("Sent focused window event to {count} clients"),
                                Err(err) => info!("No sasha clients received focused window event: {err}")
                            }
                        }
                    }
                    None => {
                        let sevt = SashaEvent::SashaWindowFocusedChanged { id: None , window_name: "None".to_string()};
                        tx.send(sevt)?;
                    }
                }
            }
            NiriEvent::WindowOpenedOrChanged { window } => {
                window_store.map.insert(window.id, window.title.clone());
            }
            NiriEvent::WorkspaceActivated { id } => {
                let sevt = SashaEvent::SashaWorkspaceActivated { id: id };
                tx.send(sevt)?;
            }
        }
    }
    Ok(())
}
