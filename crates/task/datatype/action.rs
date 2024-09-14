//! Operation [`Request`], [`Response`] and [`Manifest`] types.
//!
//! [`Request`]: ActionRequest
//! [`Response`]: ActionResponse
//! [`Manifest`]: ActionManifest

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionRequest {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionResponse {}

/// Associated action metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[must_use = "manifests do nothing unless you serialize them"]
pub struct ActionManifest {
    pub name: String,
}

impl ActionManifest {
    /// Returns a new [`ActionManifest`].
    ///
    /// Used for testing.
    #[inline]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }

    // pub fn index() -> Index {}
}
