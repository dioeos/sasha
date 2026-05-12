use serde::{Deserialize, Serialize};

#[derive(serde::Serialize, Deserialize, Debug, Clone)]
pub enum SashaEvent {
    SashaWorkspacesChanged {
        sasha_workspaces: Vec<SashaWorkspace>
    },
    SashaWindowsChanged {
        sasha_windows: Vec<SashaWindow>
    },
    SashaWindowFocusedChanged {
        id: Option<u64>,
        window_name: String
    },
    SashaWorkspaceActivated {
        idx: u64,
    },
    SashaWindowOpenedOrChanged {
        id: u64,
        window_name: String
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SashaWorkspace {
    pub id: u64,
    pub idx: u64,
    pub name: Option<String>,
    pub monitor: String,
    pub is_active: bool,
    pub is_focused: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SashaWindow {
    id: u64,
    title: String,
    app_id: String,
    workspace_id: Option<u64>,
    is_focused: bool
}


