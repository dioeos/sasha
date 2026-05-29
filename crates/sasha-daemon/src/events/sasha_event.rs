/* Events that the sasha daemon broadcasts*/
use serde::{Deserialize, Serialize};
use crate::niri::{NiriEvent, NiriWorkspace, NiriWindow};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum SashaEvent {
    Ok {
        msg: String
    },
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
        sasha_workspace: SashaWorkspace
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

impl From<&NiriWorkspace> for SashaWorkspace {
    fn from(workspace: &NiriWorkspace) -> Self {
        Self {
            id: workspace.id,
            idx: workspace.idx,
            name: workspace.name.clone(),
            monitor: workspace.output.clone(),
            is_active: workspace.is_active,
            is_focused: workspace.is_focused
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SashaWindow {
    pub id: u64,
    pub title: String,
    app_id: String,
    workspace_id: Option<u64>,
    is_focused: bool
}

impl From<&NiriWindow> for SashaWindow {
    fn from(window: &NiriWindow) -> Self {
        Self {
            id: window.id,
            title: window.title.clone(),
            app_id: window.app_id.clone(),
            workspace_id: window.workspace_id,
            is_focused: window.is_focused
        }
    }
}
