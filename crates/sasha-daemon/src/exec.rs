use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufReader, BufWriter};
use tokio::sync::broadcast;
use tokio::net::UnixStream;
use tracing::info;

use crate::niri::{NiriEvent};


use crate::events::{self, SashaEvent};

pub async fn read_niri_events(tx: broadcast::Sender<SashaEvent>) -> anyhow::Result<()> {
    let mut window_store = WindowStore::new();
    let mut workspace_store = WorkspaceStore::new();

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
                workspace_store.replace_all(workspaces);

                let mut sasha_workspaces = Vec::new();

                for (key, workspace) in &workspace_store.map {
                    sasha_workspaces.push(events::SashaWorkspace {
                        id: *key,
                        idx: workspace.idx,
                        name: workspace.name.clone(),
                        monitor: workspace.output.clone(),
                        is_active: workspace.is_active,
                        is_focused: workspace.is_focused
                    });
                }

                sasha_workspaces.sort_by_key(|workspace| workspace.idx);

                let sevt = SashaEvent::SashaWorkspacesChanged { sasha_workspaces: sasha_workspaces };
                if let Err(err) = tx.send(sevt) {
                    tracing::warn!("No Sasha clients connected yet: {err}");
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
                        if let Err(err) = tx.send(sevt) {
                            tracing::warn!("No Sasha clients connected yet: {err}");
                        }
                    }
                }
            }
            NiriEvent::WindowOpenedOrChanged { window } => {
                window_store.map.insert(window.id, window.title.clone());
                let sevt = SashaEvent::SashaWindowOpenedOrChanged { id: window.id, window_name: window.title };
                if let Err(err) = tx.send(sevt) {
                    tracing::warn!("No Sasha clients connected yet: {err}");
                }
            }
            NiriEvent::WorkspaceActivated { id } => {
                if let Some(idx) = workspace_store.get_workspace_idx(&id) {
                    let sevt = SashaEvent::SashaWorkspaceActivated { idx: *idx };
                    if let Err(err) = tx.send(sevt) {
                        tracing::warn!("No Sasha clients connected yet: {err}");
                    }
                }
            }
        }
    }
    Ok(())
}
