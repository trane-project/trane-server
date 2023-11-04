use parking_lot::Mutex;
use std::collections::HashMap;
use trane::Trane;

use crate::{config::ServerConfig, error::ServerError};

/// Holds the state of the server.
pub struct Server {
    /// The server configuration.
    pub config: ServerConfig,

    /// A map referencing all the opened libraries.
    pub opened_libraries: Mutex<HashMap<String, Trane>>,
}

impl Server {
    /// Opens the library with the given ID.
    pub fn open_library(&self, library_id: &str) -> Result<(), ServerError> {
        // Check if the library is already opened.
        if self.opened_libraries.lock().contains_key(library_id) {
            return Ok(());
        }

        // Check if the library exists in the library directory.
        let library_path = self.config.library_directory.join(library_id);
        if !library_path.exists() {
            return Err(ServerError::LibraryNotFound(library_id.to_string()));
        }

        // Open the library and add it to the map.
        let library = Trane::new(&library_path, &library_path).map_err(|err| {
            ServerError::InternalError(format!(
                "cannot open library with ID {}: {}",
                library_id, err
            ))
        })?;
        self.opened_libraries
            .lock()
            .insert(library_id.to_string(), library);
        Ok(())
    }

    /// Closes the library with the given ID. If the library does not exist or is not opened, this
    /// does nothing.
    pub fn close_library(&self, library_id: &str) -> Result<(), ServerError> {
        self.opened_libraries.lock().remove(library_id);
        Ok(())
    }
}
