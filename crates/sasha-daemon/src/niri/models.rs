/* Models to mirror the Niri IPC structures */
use std::fmt;

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum NiriEvent {
    Ok (String),

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
    },
}

impl fmt::Display for NiriEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NiriEvent::Ok(msg) => { 
                write!(
                    f,
                    "Ok: ({})",
                    msg
                )
            }
            NiriEvent::WorkspacesChanged { workspaces } => {
                write!(
                    f,
                    "WorkspacesChanged: ({} workspaces)",
                    workspaces.len()
                )
            }
            NiriEvent::WindowsChanged { windows } => {
                write!(
                    f,
                    "WindowsChanged: ({} windows)",
                    windows.len()
                )
            }
            NiriEvent::WindowFocusChanged { id } => {
                match id {
                    Some(id) => {
                        write!(f, "WindowFocusChanged: {}", id)
                    }
                    None => {
                        write!(f, "WindowFocusChanged: None")
                    }
                }
            }
            NiriEvent::WindowOpenedOrChanged { window } => {
                write!(
                    f,
                    "WindowOpenedOrChanged: {}",
                    window
                )
            }
            NiriEvent::WorkspaceActivated { id } => {
                write!(
                    f,
                    "WorkspaceActivated: {}",
                    id
                )
            }
        }
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

impl fmt::Display for NiriWindow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "NiriWindow: [{}], title: {}, is_focused: {}",
            self.id,
            self.title,
            self.is_focused
        )

    }
}
