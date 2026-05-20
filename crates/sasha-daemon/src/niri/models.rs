/* Models to mirror the Niri IPC structures */
#[derive(serde::Deserialize, Debug)]
pub enum NiriEvent {
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
pub struct NiriWorkspace {
    pub id: u64,
    pub idx: u64,
    pub name: Option<String>,
    pub output: String,
    is_urgent: bool,
    pub is_active: bool,
    pub is_focused: bool,
    active_window_id: Option<u64>
}

#[derive(serde::Deserialize, Debug)]
pub struct NiriWindow {
    pub id: u64,
    pub title: String,
    pub app_id: String,
    pid: u64,
    pub workspace_id: Option<u64>,
    pub is_focused: bool,
    is_floating: bool,
    is_urgent: bool
}
