use std::sync::Arc;

use crate::niri::NiriConnector;
use crate::stores::MarkStore;

pub struct CommandHandler {
    mark_store: MarkStore,
    niri_connector: Arc<NiriConnector>
}
