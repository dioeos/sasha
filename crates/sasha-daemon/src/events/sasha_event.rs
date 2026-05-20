/* Events that the sasha daemon broadcasts*/
use serde::{Deserialize, Serialize};
use crate::niri::{NiriEvent, NiriWorkspace, NiriWindow};

#[derive(Deserialize, Serialize, Debug, Clone)]
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


//pure conversions: -> Contain enough data already to convert to SashaEvent
// WorkspacesChanged
// WindowsChanged
// WindowOpenedOrChanged
//
impl From<NiriEvent::WorkspacesChanged> for SashaEvent::SashaWorkspacesChanged {
    fn from(workspaces_evt: NiriEvent::WorkspacesChanged) -> Self {

    }
}

//non pure conversions: -> Require querying state
// WindowFocusChanged
// WorkspaceActivated

// impl From<NiriEvent> for SashaEvent {
//     fn from(event: NiriEvent) -> Self {
//         match event {
//             NiriEvent::WorkspacesChanged { workspaces } => {
//                 SashaEvent::SashaWorkspacesChanged {
//                     sasha_workspaces: workspaces
//                         .into_iter()
//                         .map(|workspace| SashaWorkspace::from(workspace))
//                         .collect()
//                 }
//             }
//             NiriEvent::WindowsChanged { windows } => {
//                 SashaEvent::SashaWindowsChanged {
//                     sasha_windows: windows
//                         .into_iter()
//                         .map(|window| SashaWindow::from(window))
//                         .collect()
//                 }
//             }
//             NiriEvent::WindowFocusChanged { id } => {
//                 match id {
//                     Some(id) => {
//                         if let Some(name) = window
//                     }
//                 }
//
//             }
//
//         }
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SashaWorkspace {
pub id: u64,
    pub idx: u64,
    pub name: Option<String>,
    pub monitor: String,
    pub is_active: bool,
    pub is_focused: bool
}

impl From<NiriWorkspace> for SashaWorkspace {
    fn from(workspace: NiriWorkspace) -> Self {
        Self {
            id: workspace.id,
            idx: workspace.idx,
            name: workspace.name,
            monitor: workspace.output,
            is_active: workspace.is_active,
            is_focused: workspace.is_focused
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SashaWindow {
    id: u64,
    title: String,
    app_id: String,
    workspace_id: Option<u64>,
    is_focused: bool
}

impl From<NiriWindow> for SashaWindow {
    fn from(window: NiriWindow) -> Self {
        Self {
            id: window.id,
            title: window.title,
            app_id: window.app_id,
            workspace_id: window.workspace_id,
            is_focused: window.is_focused
        }
    }
}
