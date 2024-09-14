use serde::{Deserialize, Serialize};

/// Associated service metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[must_use = "manifests do nothing unless you serialize them"]
pub struct ServiceManifest {
    pub name: String,
}

impl ServiceManifest {
    /// Returns a new [`ServiceManifest`].
    ///
    /// Used for testing.
    #[inline]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }
}
