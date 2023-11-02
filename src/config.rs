use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Holds the configuration for the server.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    /// The address to bind to.
    pub address: String,

    /// The port to bind to.
    pub port: u16,

    /// The directory in which the Trane libraries to serve are stored in.
    pub library_directory: PathBuf,
}
