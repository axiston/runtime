//! Service [`Router`] and declarative [`Layers`].
//!

use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

use tower::load::Load;
use tower::{Layer, Service, ServiceExt};

use crate::context::{TaskError, TaskRequest, TaskResponse};
use crate::handler::future::TaskFuture;
use crate::handler::metric::TaskMetric;
use crate::handler::TaskHandler;
use crate::routing::builder::LayerBuilder;
pub use crate::routing::index::Index;
pub use crate::routing::layers::Layers;

mod builder;
mod index;
mod layers;

/// Collection of all registered [`TaskHandler`]s.
pub struct Router<M, T, U> {
    inner: Arc<Mutex<RouterInner<M, T, U>>>,
}

struct RouterInner<M, T, U> {
    builder: LayerBuilder<M, T, U>,
    tasks: HashMap<Index, TaskHandler<M, T, U>>,
}

impl<M, T, U> Router<M, T, U> {
    /// Returns an empty [`Router`].
    pub fn new(layers: Layers) -> Self {
        let inner = Arc::new(Mutex::new(RouterInner {
            builder: LayerBuilder::new(layers),
            tasks: HashMap::default(),
        }));

        Self { inner }
    }

    /// Inserts or replaces default [`Layers`].
    #[inline]
    pub fn with_layers(&mut self, layers: Layers) {
        let mut guard = self.inner.lock().unwrap();
        let _ = guard.builder.replace_layers(layers);
    }

    /// Inserts or replaces [`TaskHandler`] at the [`Index`].
    #[inline]
    pub fn route(&mut self, index: Index, handler: TaskHandler<M, T, U>) {
        let mut guard = self.inner.lock().unwrap();
        let _ = guard.tasks.insert(index, handler);
    }

    /// Returns a [`TaskHandler`] corresponding to the [`Index`].
    #[inline]
    pub fn find(&self, index: &Index) -> Option<TaskHandler<M, T, U>>
    where
        M: Clone,
        T: 'static,
        U: 'static,
    {
        let guard = self.inner.lock().unwrap();
        let handler = guard.tasks.get(index).cloned();
        handler.map(|handler| guard.builder.apply(handler))
    }

    /// Returns a [`TaskHandler`] with corresponding to the [`Index`].
    #[inline]
    pub fn find_layered(&self, index: &Index, layers: Layers) -> Option<TaskHandler<M, T, U>>
    where
        M: Clone,
        T: 'static,
        U: 'static,
    {
        let guard = self.inner.lock().unwrap();
        let handler = guard.tasks.get(index).cloned();
        handler.map(|handler| guard.builder.apply_layers(handler, layers))
    }

    /// Returns the number of [`TaskHandler`]s in the [`Router`].
    #[inline]
    pub fn len(&self) -> usize {
        let guard = self.inner.lock().unwrap();
        guard.tasks.len()
    }

    /// Returns `true` if the [`Router`] contains no [`TaskHandler`]s.
    #[inline]
    pub fn is_empty(&self) -> bool {
        let guard = self.inner.lock().unwrap();
        guard.tasks.is_empty()
    }
}

impl<M, T, U> Default for Router<M, T, U> {
    fn default() -> Self {
        let inner = Arc::new(Mutex::new(RouterInner {
            builder: LayerBuilder::default(),
            tasks: HashMap::default(),
        }));

        Self { inner }
    }
}

impl<M, T, U> Clone for Router<M, T, U>
where
    M: Clone,
{
    fn clone(&self) -> Self {
        let inner = self.inner.clone();
        Self { inner }
    }
}

impl<M, T, U> fmt::Debug for Router<M, T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Router").finish_non_exhaustive()
    }
}

impl<M, T, U> Service<TaskRequest<T>> for Router<M, T, U>
where
    M: Clone + Send + 'static,
    T: Send + 'static,
    U: 'static,
{
    type Response = TaskResponse<U>;
    type Error = TaskError;
    type Future = TaskFuture<U>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    #[inline]
    fn call(&mut self, req: TaskRequest<T>) -> Self::Future {
        match self.find(req.index()) {
            Some(handler) => {
                let fut = async move { handler.oneshot(req).await };
                TaskFuture::new(fut)
            }
            None => {
                // TODO: Use a properly formatted error.
                let fut = async { Err(TaskError::new(())) };
                TaskFuture::new(fut)
            }
        }
    }
}

impl<M, T, U> Load for Router<M, T, U> {
    type Metric = TaskMetric;

    #[inline]
    fn load(&self) -> Self::Metric {
        // TODO: Call .load() of the underlying service.
        TaskMetric::new()
    }
}

#[cfg(test)]
mod test {
    use crate::routing::{Layers, Router};
    use crate::Result;

    #[test]
    fn build() -> Result<()> {
        let layers = Layers::new();
        let _ = Router::<(), i32, i32>::new(layers);
        Ok(())
    }
}
