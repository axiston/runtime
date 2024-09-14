use crate::compose::LayersConfig;

/// [`LayersConfig`] builder.
#[derive(Debug, Default, Clone)]
pub struct LayersBuilder {}

impl LayersBuilder {
    /// Returns a new [`LayersBuilder`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new [`LayersConfig`].
    #[inline]
    pub fn build(self) -> LayersConfig {
        LayersConfig {}
    }
}

#[cfg(test)]
mod test {
    use crate::compose::{LayersBuilder, LayersConfig};

    #[test]
    fn from_default() {
        let _ = LayersConfig::new();
    }

    #[test]
    fn from_builder() {
        let _ = LayersBuilder::new();
    }
}
