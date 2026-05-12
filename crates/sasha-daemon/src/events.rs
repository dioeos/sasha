use serde::{Deserialize, Serialize};

#[derive(serde::Serialize, Deserialize, Debug, Clone)]
pub enum SashaEvent {
    SashaWorkspacesChanged {
        sasha_workspace: Vec<SashaWorkspace>
    },
    SashaWindowsChanged {
        sasha_windows: Vec<SashaWindow>
    },
    SashaWindowFocusedChanged {
        id: Option<u64>,
        window_name: String
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SashaWorkspace {
    id: u64,
    name: Option<String>,
    monitor: String,
    is_active: bool,
    is_focused: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SashaWindow {
    id: u64,
    title: String,
    app_id: String,
    workspace_id: Option<u64>,
    is_focused: bool
}


