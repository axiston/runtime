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
pub struct Layers {}

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
pub struct LayersBuilder {}

impl LayersBuilder {
    /// Returns a new [`LayersBuilder`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new [`Layers`].
    #[inline]
    pub fn build(self) -> Layers {
        Layers {}
    }
}

#[cfg(test)]
mod test {
    use crate::routing::layer_compose::{LayerCompose, Layers, LayersBuilder};
    use crate::Result;

    #[test]
    fn from_default() -> Result<()> {
        let config = Layers::new();
        let _compose = LayerCompose::new(config);
        Ok(())
    }

    #[test]
    fn from_builder() -> Result<()> {
        let config = LayersBuilder::new().build();
        let _compose = LayerCompose::new(config);
        Ok(())
    }
}
