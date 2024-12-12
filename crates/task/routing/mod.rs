//! [`Router`], [`RouteIndex`] and [`compose`] utilities.
//!
//! [`compose`]: Layers

use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use crate::routing::index::{RouteIndex, ServiceIndex};
use crate::routing::layers::{LayerCompose, Layers};
use crate::routing::manifest::{RouteManifest, ServiceManifest};
pub use crate::routing::route::Route;

pub mod index;
pub mod layers;
pub mod manifest;
mod route;

/// TODO.
pub type RouteRequest = ();

/// TODO.
pub type RouteResponse = ();

/// TODO.
#[must_use = "routes do nothing unless you use them"]
pub struct Router<T = RouteRequest, U = RouteResponse> {
    router_inner: Arc<RouterInner<T, U>>,
}

struct RouterInner<T, U> {
    layer_compose: LayerCompose,
    service_manifests: HashMap<ServiceIndex, ServiceManifest>,
    route_handlers: HashMap<RouteIndex, Route<T, U>>,
}

impl<T, U> Router<T, U> {
    /// Returns an empty [`Router`].
    #[inline]
    pub fn new(layers: Layers) -> Self {
        let router_inner = RouterInner {
            layer_compose: LayerCompose::new(layers),
            service_manifests: HashMap::default(),
            route_handlers: HashMap::new(),
        };

        Self {
            router_inner: Arc::new(router_inner),
        }
    }

    /// Overrides the default value of [`Router`]`::layer_compose`.
    pub fn with_layers(self, layers: Layers) -> Self {
        let mut inner = Arc::try_unwrap(self.router_inner)
            .unwrap_or_else(|router_handler| (*router_handler).clone());
        inner.layer_compose = LayerCompose::new(layers);

        Self {
            router_inner: Arc::new(inner),
        }
    }

    /// Registers another [`ServiceManifest`] by its [`ServiceIndex`].
    pub fn with_service(
        self,
        service_index: ServiceIndex,
        service_manifest: ServiceManifest,
    ) -> Self {
        let mut inner = Arc::try_unwrap(self.router_inner)
            .unwrap_or_else(|router_handler| (*router_handler).clone());
        let _ = inner
            .service_manifests
            .insert(service_index, service_manifest);

        Self {
            router_inner: Arc::new(inner),
        }
    }

    /// Registers another [`Route`] by its [`RouteIndex`].
    pub fn with_route(self, route_index: RouteIndex, route: Route<T, U>) -> Self {
        let mut inner = Arc::try_unwrap(self.router_inner)
            .unwrap_or_else(|router_handler| (*router_handler).clone());
        let _ = inner.route_handlers.insert(route_index, route);

        Self {
            router_inner: Arc::new(inner),
        }
    }

    // TODO: Method to return the whole registry.
    // TODO: Method to execute a single route.
}

impl<T, U> fmt::Debug for Router<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Router").finish_non_exhaustive()
    }
}

impl<T, U> Default for Router<T, U> {
    fn default() -> Self {
        let router_handler = RouterInner {
            layer_compose: LayerCompose::default(),
            service_manifests: HashMap::default(),
            route_handlers: HashMap::default(),
        };

        Self {
            router_inner: Arc::new(router_handler),
        }
    }
}

impl<T, U> Clone for Router<T, U> {
    fn clone(&self) -> Self {
        Self {
            router_inner: self.router_inner.clone(),
        }
    }
}

impl<T, U> Clone for RouterInner<T, U> {
    fn clone(&self) -> Self {
        Self {
            layer_compose: self.layer_compose.clone(),
            service_manifests: self.service_manifests.clone(),
            route_handlers: self.route_handlers.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::routing::{Layers, Router};
    use crate::Result;

    #[test]
    fn build_default_router() -> Result<()> {
        let _router: Router = Router::new(Layers::new());
        Ok(())
    }
}
