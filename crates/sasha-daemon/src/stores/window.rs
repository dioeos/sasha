use std::{collections::HashMap};
use crate::niri::{NiriWindow};

pub struct WindowStore {
    pub map: HashMap<u64, NiriWindow>
}

impl WindowStore {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub fn replace_all(&mut self, windows: Vec<NiriWindow>) {
        self.map = windows
            .into_iter()
            .map(|window| (window.id, window))
            .collect()
    }

    pub fn get_window_name(&self, key: &u64) -> Option<&str> {
        self.map.get(key).map(|window| window.title.as_str())
    }
}
