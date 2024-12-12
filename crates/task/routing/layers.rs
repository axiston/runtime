//! [`LayerCompose`], [`Layers`] and its [`LayersBuilder`].

use serde::{Deserialize, Serialize};

/// TODO.
#[derive(Debug, Default, Clone)]
pub struct LayerCompose {
    layers: Option<Layers>,
}

impl LayerCompose {
    /// Returns a new [`LayerCompose`].
    #[inline]
    pub fn new(layers: Layers) -> Self {
        Self {
            layers: Some(layers),
        }
    }
}

/// Declarative `tower::`[`Layer`]s configuration.
///
/// [`Layer`]: tower::Layer
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Layers {
    timeout_policy: Option<()>,
    retry_policy: Option<()>,
}

impl Layers {
    /// Returns a new [`Layers`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new [`LayersBuilder`].
    #[inline]
    pub fn builder() -> LayersBuilder {
        LayersBuilder::new()
    }
}

/// [`Layers`] builder.
#[derive(Debug, Default, Clone)]
pub struct LayersBuilder {
    timeout_policy: Option<()>,
    retry_policy: Option<()>,
}

impl LayersBuilder {
    /// Returns a new [`LayersBuilder`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Overrides the default value of [`LayersBuilder`]`::timeout_policy`.
    pub fn with_timeout_policy(mut self, timeout_policy: ()) -> Self {
        self.timeout_policy = Some(timeout_policy);
        self
    }

    /// Overrides the default value of [`LayersBuilder`]`::retry_policy`.
    pub fn with_retry_policy(mut self, retry_policy: ()) -> Self {
        self.retry_policy = Some(retry_policy);
        self
    }

    /// Returns a new [`Layers`].
    pub fn build(self) -> Layers {
        Layers {
            timeout_policy: self.timeout_policy,
            retry_policy: self.retry_policy,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::routing::layers::{LayerCompose, Layers, LayersBuilder};
    use crate::Result;

    #[test]
    fn with_default_layers() -> Result<()> {
        let config = Layers::new();
        let _compose = LayerCompose::new(config);
        Ok(())
    }

    #[test]
    fn from_layers_builder() -> Result<()> {
        let config = LayersBuilder::new().build();
        let _compose = LayerCompose::new(config);
        Ok(())
    }
}
