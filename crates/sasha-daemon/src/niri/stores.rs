/* Stores to store Niri state */

use std::{collections::HashMap, hash::Hash};

use super::models::{NiriWorkspace};

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

pub struct WorkspaceStore {
    pub map: HashMap<u64, NiriWorkspace>
}

impl WorkspaceStore {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub fn replace_all(&mut self, workspaces: Vec<NiriWorkspace>) {
        self.map = workspaces
            .into_iter()
            .map(|workspace| (workspace.id, workspace))
            .collect()
    }

    pub fn get_workspace_idx(&self, key: &u64) -> Option<&u64> {
        self.map.get(key).map(|workspace| &workspace.idx)
    }

}
