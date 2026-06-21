use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RequestEvent {
    MarkWindow {
        slot: u8,
    },
    FocusWindow {
        slot: u8
    }
}
