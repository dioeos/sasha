use std::{collections::HashMap};

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
