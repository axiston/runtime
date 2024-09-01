//! Associated [`ConditionManifest`] types.
//!

use serde::{Deserialize, Serialize};

/// Associated trigger metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[must_use = "manifests do nothing unless you serialize them"]
pub struct ConditionManifest {}

impl ConditionManifest {
    /// Returns a new [`ConditionManifest`].
    ///
    /// Used for testing.
    #[inline]
    pub fn new(name: &str) -> Self {
        Self {}
    }
}
