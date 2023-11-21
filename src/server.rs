use parking_lot::Mutex;
use std::{collections::HashMap, path::Path, sync::Arc};
use trane::Trane;

use crate::{config::ServerConfig, error::ServerError};

#[derive(Clone)]
/// A remote Trane instance.
pub struct RemoteTrane {
    // The ID of the library.
    pub library_id: String,

    /// The Trane instance.
    pub trane: Arc<Mutex<Trane>>,
}

impl RemoteTrane {
    /// Creates a new remote Trane instance.
    pub fn new(library_id: &str, library_path: &Path) -> Result<Self, ServerError> {
        let instance = Arc::new(Mutex::new(
            Trane::new(&library_path, &library_path).map_err(|err| {
                ServerError::InternalError(format!(
                    "cannot open library with ID {}: {}",
                    library_id, err
                ))
            })?,
        ));
        Ok(Self {
            library_id: library_id.to_string(),
            trane: instance,
        })
    }
}

/// Holds the state of the server.
pub struct Server {
    /// The server configuration.
    pub config: ServerConfig,

    /// A map referencing all the opened Trane libraries, indexed by their ID.
    pub remote_instances: Mutex<HashMap<String, RemoteTrane>>,
}

impl Server {
    /// Checks if the library with the given ID is already opened. If `strict` is `true`, this
    /// returns an error if the library does not exist.
    pub fn check_library(&self, library_id: &str, strict: bool) -> Result<bool, ServerError> {
        if !self.remote_instances.lock().contains_key(library_id) {
            if strict {
                return Err(ServerError::LibraryNotFound(library_id.to_string()));
            } else {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// Opens the library with the given ID.
    pub fn open_library(&self, library_id: &str) -> Result<(), ServerError> {
        // Check if the library exists in the library directory.
        if self.check_library(library_id, false)? {
            return Ok(());
        }
        let library_path = self.config.library_directory.join(library_id);
        if !library_path.exists() {
            return Err(ServerError::LibraryNotFound(library_id.to_string()));
        }

        // Open the library and add it to the map.
        self.remote_instances.lock().insert(
            library_id.to_string(),
            RemoteTrane::new(library_id, &library_path)?,
        );
        Ok(())
    }

    /// Closes the library with the given ID. If the library does not exist or is not opened, this
    /// does nothing.
    pub fn close_library(&self, library_id: &str) -> Result<(), ServerError> {
        self.remote_instances.lock().remove(library_id);
        Ok(())
    }

    /// Returns the remote instance that corresponds to the library with the given ID.
    pub fn get_library(&self, library_id: &str) -> Result<RemoteTrane, ServerError> {
        let _ = self.check_library(library_id, true)?;
        Ok(self
            .remote_instances
            .lock()
            .get(library_id)
            .unwrap()
            .clone())
    }
}
