//! Associated [`OperationManifest`] types.
//!

use serde::{Deserialize, Serialize};

/// Associated action metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[must_use = "manifests do nothing unless you serialize them"]
pub struct OperationManifest {}

impl OperationManifest {
    /// Returns a new [`OperationManifest`].
    ///
    /// Used for testing.
    #[inline]
    pub fn new(name: &str) -> Self {
        Self {}
    }
}
