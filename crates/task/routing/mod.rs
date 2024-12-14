//! [`Router`], [`RouteIndex`] and [`compose`] utilities.
//!
//! [`compose`]: Layers

use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use crate::context::{TaskRequest, TaskResponse};
use crate::routing::index::{RouteIndex, ServiceIndex};
use crate::routing::layers::{LayerCompose, Layers};
use crate::routing::manifest::{RouteManifest, ServiceManifest};
pub use crate::routing::route::Route;
use crate::{Registry, Result};

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
    inner: Arc<RouterInner<T, U>>,
}

// TODO: Should route manifest be inside of route handler?

struct RouterInner<T, U> {
    layer_compose: Option<LayerCompose>,
    service_manifests: HashMap<ServiceIndex, ServiceManifest>,
    routes: HashMap<RouteIndex, Route<T, U>>,
}

impl<T, U> Router<T, U> {
    /// Returns an empty [`Router`].
    #[inline]
    pub fn new(layers: Layers) -> Self {
        let router_inner = RouterInner {
            layer_compose: Some(LayerCompose::new(layers)),
            service_manifests: HashMap::default(),
            routes: HashMap::new(),
        };

        Self {
            inner: Arc::new(router_inner),
        }
    }

    /// Overrides the default value of [`Router`]`::layer_compose`.
    pub fn with_layers(self, layers: Layers) -> Self {
        let mut inner = Arc::try_unwrap(self.inner).unwrap_or_else(|x| (*x).clone());
        inner.layer_compose = Some(LayerCompose::new(layers));

        Self {
            inner: Arc::new(inner),
        }
    }

    /// Registers another [`ServiceManifest`] by its [`ServiceIndex`].
    pub fn with_service(
        self,
        service_index: ServiceIndex,
        service_manifest: ServiceManifest,
    ) -> Self {
        let mut inner = Arc::try_unwrap(self.inner).unwrap_or_else(|x| (*x).clone());
        let _ = inner
            .service_manifests
            .insert(service_index, service_manifest);

        Self {
            inner: Arc::new(inner),
        }
    }

    /// Registers another [`Route`] by its [`RouteIndex`].
    pub fn with_route(self, route_index: RouteIndex, route: Route<T, U>) -> Self {
        let mut inner = Arc::try_unwrap(self.inner).unwrap_or_else(|x| (*x).clone());
        let _ = inner.routes.insert(route_index, route);

        Self {
            inner: Arc::new(inner),
        }
    }

    // TODO: Method to return the whole registry.
    // TODO: Method to execute a single route.

    /// Returns the reference to the [`ServiceManifest`].
    pub fn get_service_manifest(&self, service_index: &ServiceIndex) -> Option<&ServiceManifest> {
        self.inner.service_manifests.get(service_index)
    }

    /// Returns the reference to the [`RouteManifest`].
    pub fn get_route_manifest(&self, route_index: &RouteIndex) -> Option<&RouteManifest> {
        let route = self.inner.routes.get(route_index);
        route.map(|x| &x.route_handler.route_manifest)
    }

    /// Returns all [`ServiceManifest`]s and [`RouteManifest`]s.
    pub fn get_registry(&self) -> Registry {
        let routes = self.inner.routes.iter();
        Registry {
            services: self.inner.service_manifests.clone(),
            routes: routes
                .map(|(i, r)| (i.clone(), r.route_handler.route_manifest.clone()))
                .collect(),
        }
    }

    /// TODO.
    pub async fn route(
        &self,
        route_index: &RouteIndex,
        task_request: TaskRequest<T>,
    ) -> Result<TaskResponse<U>>
    where
        T: 'static,
        U: 'static,
    {
        let route_handler = self.inner.routes.get(route_index);
        let route_handler = route_handler.ok_or_else(|| route_index.clone())?;
        let layer_compose = self.inner.layer_compose.as_ref();
        route_handler.route(task_request, layer_compose).await
    }
}

impl<T, U> fmt::Debug for Router<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Router").finish_non_exhaustive()
    }
}

impl<T, U> Default for Router<T, U> {
    fn default() -> Self {
        let router_handler = RouterInner {
            layer_compose: None,
            service_manifests: HashMap::default(),
            routes: HashMap::default(),
        };

        Self {
            inner: Arc::new(router_handler),
        }
    }
}

impl<T, U> Clone for Router<T, U> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T, U> Clone for RouterInner<T, U> {
    fn clone(&self) -> Self {
        Self {
            layer_compose: self.layer_compose.clone(),
            service_manifests: self.service_manifests.clone(),
            routes: self.routes.clone(),
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
