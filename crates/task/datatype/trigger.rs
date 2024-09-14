//! Condition [`Request`], [`Response`] and [`Manifest`] types.
//!
//! [`Request`]: TriggerRequest
//! [`Response`]: TriggerResponse
//! [`Manifest`]: TriggerManifest

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TriggerRequest {}

#[derive(Debug, Serialize, Deserialize)]
pub struct TriggerResponse {
    pub should_trigger: bool,
    pub ignore_retry_ms: u32,
}

/// Associated trigger metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[must_use = "manifests do nothing unless you serialize them"]
pub struct TriggerManifest {
    pub name: String,
}

impl TriggerManifest {
    /// Returns a new [`TriggerManifest`].
    ///
    /// Used for testing.
    #[inline]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }
}
