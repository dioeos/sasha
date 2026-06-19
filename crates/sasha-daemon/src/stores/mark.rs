use std::{collections::HashMap};

pub struct MarkStore {
    pub map: HashMap<u8, u64>
}

impl MarkStore {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub fn get_window_id(&self, key: &u8) -> Option<&u64> {
        self.map.get(key)
    }
}
