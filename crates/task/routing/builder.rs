use std::fmt;
use std::marker::PhantomData;

use tower::{ServiceBuilder, ServiceExt};

use crate::context::{TaskRequest, TaskResponse};
use crate::handler::TaskHandler;
use crate::routing::Layers;
use crate::Result;

/// Declarative `tower::`[`Layer`] builder.
///
/// [`Layer`]: tower::Layer
pub struct LayerBuilder<M, T, U> {
    layers: Option<Layers>,
    manifest: PhantomData<M>,
    request: PhantomData<T>,
    response: PhantomData<U>,
}

impl<M, T, U> LayerBuilder<M, T, U> {
    /// Returns a new [`LayerBuilder`] with default [`Layers`].
    #[inline]
    pub fn new(layers: Layers) -> Self {
        Self {
            layers: Some(layers),
            manifest: PhantomData,
            request: PhantomData,
            response: PhantomData,
        }
    }

    /// Inserts or replaces default [`Layers`].
    #[inline]
    pub fn replace_layers(&mut self, layers: Layers) -> Option<Layers> {
        self.layers.replace(layers)
    }

    /// Applies default [`Layers`] to the `handler`.
    pub fn apply(&self, handler: TaskHandler<M, T, U>) -> TaskHandler<M, T, U>
    where
        T: 'static,
        U: 'static,
    {
        let builder = ServiceBuilder::new();
        handler.map(|svc| builder.service(svc))
    }

    /// Merges default [`Layers`] with provided and applies to the `handler`.
    pub fn apply_layers(
        &self,
        handler: TaskHandler<M, T, U>,
        layers: Layers,
    ) -> TaskHandler<M, T, U>
    where
        T: 'static,
        U: 'static,
    {
        let builder = ServiceBuilder::new();
        handler.map(|svc| builder.service(svc))
    }

    /// Applies specified [`Layer`]s to the [`TaskHandler`].
    pub async fn execute(
        &self,
        handler: TaskHandler<M, T, U>,
        request: TaskRequest<T>,
    ) -> Result<TaskResponse<U>>
    where
        T: 'static,
        U: 'static,
    {
        let handler = self.apply(handler);
        let response = handler.oneshot(request).await;
        response.map_err(Into::into)
    }

    /// Applies specified [`Layer`]s to the [`TaskHandler`].
    pub async fn execute_with_layers(
        &self,
        handler: TaskHandler<M, T, U>,
        request: TaskRequest<T>,
        layers: Layers,
    ) -> Result<TaskResponse<U>>
    where
        T: 'static,
        U: 'static,
    {
        let handler = self.apply_layers(handler, layers);
        let response = handler.oneshot(request).await;
        response.map_err(Into::into)
    }
}

impl<M, T, U> Default for LayerBuilder<M, T, U> {
    fn default() -> Self {
        Self {
            layers: None,
            manifest: PhantomData,
            request: PhantomData,
            response: PhantomData,
        }
    }
}

impl<M, T, U> Clone for LayerBuilder<M, T, U> {
    fn clone(&self) -> Self {
        Self {
            layers: self.layers.clone(),
            manifest: PhantomData,
            request: PhantomData,
            response: PhantomData,
        }
    }
}

impl<M, T, U> fmt::Debug for LayerBuilder<M, T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LayerBuilder").finish_non_exhaustive()
    }
}
