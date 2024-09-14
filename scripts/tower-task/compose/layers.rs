#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::compose::LayersBuilder;

/// [`LayerCompose`] configuration for a single service call.
///
/// [`LayerCompose`]: crate::service::LayerCompose
#[derive(Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LayersConfig {}

impl LayersConfig {
    /// Returns a new [`LayersConfig`].
    #[inline]
    pub fn new() -> Self {
        Self::new()
    }

    /// Returns a new [`LayersBuilder`].
    #[inline]
    pub fn builder() -> LayersBuilder {
        LayersBuilder::new()
    }
}
