use std::{collections::HashMap};
use crate::niri::{NiriWorkspace};

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
