//! [`Router`], [`RouteIndex`] and [`compose`] utilities.
//!
//! [`compose`]: Layers

use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

use crate::handler::TaskHandler;
use crate::routing::layer_compose::LayerCompose;
pub use crate::routing::layer_compose::{Layers, LayersBuilder};
pub use crate::routing::route_index::RouteIndex;
use crate::Result;

mod index;
mod layer_compose;
mod route_index;

/// TODO.
#[must_use = "routers do nothing unless you use them"]
pub struct Router<T, U> {
    layer_compose: LayerCompose,
    route_services: HashMap<RouteIndex, TaskHandler<T, U>>,
}

impl<T, U> Router<T, U> {
    /// Returns an empty [`Router`].
    #[inline]
    pub fn new(layers: Layers) -> Self {
        Self {
            layer_compose: LayerCompose::new(layers),
            route_services: HashMap::new(),
        }
    }

    /// Overrides the default value of [`Router`]`::layers`.
    #[inline]
    pub fn with_layers(mut self, layers: Layers) -> Self {
        self.layer_compose = LayerCompose::new(layers);
        self
    }
}

impl<T, U> fmt::Debug for Router<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Router").finish_non_exhaustive()
    }
}

impl<T, U> Default for Router<T, U> {
    fn default() -> Self {
        Self {
            layer_compose: LayerCompose::default(),
            route_services: HashMap::default(),
        }
    }
}

impl<T, U> Clone for Router<T, U> {
    fn clone(&self) -> Self {
        Self {
            layer_compose: self.layer_compose.clone(),
            route_services: self.route_services.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::routing::{Layers, Router};
    use crate::Result;

    #[test]
    fn build_default_router() -> Result<()> {
        // TODO.
        // let _ = Router::new(Layers::new());
        Ok(())
    }
}
