#[derive(Clone, Debug)]
pub enum SashaEvent {
    FocusedWindow(String),
    WorksapceChanged(u32),
}
