use parking_lot::Mutex;
use std::collections::HashMap;
use trane::Trane;

use crate::config::ServerConfig;

/// Holds the state of the server.
pub struct ServerState {
    /// The server configuration.
    pub config: ServerConfig,

    pub opened_libraries: Mutex<HashMap<String, Trane>>,
}
