//! [`RouteManifest`] and [`ServiceManifest`].

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// TODO.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[must_use = "manifests do nothing unless you serialize them"]
pub struct ServiceManifest {
    pub service_id: String,
}

impl ServiceManifest {
    /// Returns a new [`ServiceManifest`].
    pub fn new() -> Self {
        todo!()
    }
}

/// TODO.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[must_use = "manifests do nothing unless you serialize them"]
pub struct RouteManifest {
    pub route_id: String,
    // version, deprecation notice
    pub service_id: String,
    pub inputs_schema: Value,
    pub outputs_schema: Value,
    pub errors_schema: Value,
}

impl RouteManifest {
    /// Returns a new [`RouteManifest`].
    pub fn new() -> Self {
        todo!()
    }
}

#[cfg(test)]
mod test {}
