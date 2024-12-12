//! [`TaskHandler`], [`TaskHandlerLayer`], its future and metrics.

use std::fmt;
use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::Arc;
use std::task::{Context, Poll};

use tower::load::Load;
use tower::util::BoxCloneService;
use tower::{Layer, Service, ServiceBuilder};

use crate::context::{TaskError, TaskRequest, TaskResponse};
use crate::handler::future::TaskFuture;
use crate::handler::metric::TaskMetrics;

pub mod future;
pub mod metric;

/// Unified `tower::`[`Service`] for executing tasks.
///
/// Opaque [`BoxCloneService`]<[`TaskRequest`], [`TaskResponse`], [`TaskError`]>.
#[must_use = "services do nothing unless you `.poll_ready` or `.call` them"]
pub struct TaskHandler<T, U> {
    inner: BoxCloneService<TaskRequest<T>, TaskResponse<U>, TaskError>,
    metrics: Arc<TaskMetrics>,
}

impl<T, U> TaskHandler<T, U> {
    /// Returns a new [`TaskHandler`].
    #[inline]
    pub fn new<S, Req>(inner: S) -> Self
    where
        T: 'static,
        U: 'static,
        S: Service<Req> + Clone + Send + 'static,
        Req: From<TaskRequest<T>> + 'static,
        S::Response: Into<TaskResponse<U>> + 'static,
        S::Error: Into<TaskError> + 'static,
        S::Future: Send + 'static,
    {
        let inner = ServiceBuilder::new()
            .map_request(From::from)
            .map_response(Into::into)
            .map_err(Into::into)
            .service(inner);

        Self {
            inner: BoxCloneService::new(inner),
            metrics: Arc::new(TaskMetrics::default()),
        }
    }

    /// Maps a `TaskHandler<T, U>` to `TaskHandler<T2, U2>` by applying a function to a contained service.
    pub fn map<T2, U2, F>(self, f: F) -> TaskHandler<T2, U2>
    where
        F: FnOnce(
            BoxCloneService<TaskRequest<T>, TaskResponse<U>, TaskError>,
        ) -> BoxCloneService<TaskRequest<T2>, TaskResponse<U2>, TaskError>,
    {
        TaskHandler {
            inner: f(self.inner),
            metrics: self.metrics,
        }
    }
}

impl<T, U> Clone for TaskHandler<T, U> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            metrics: self.metrics.clone(),
        }
    }
}

impl<T, U> fmt::Debug for TaskHandler<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TaskHandler").finish_non_exhaustive()
    }
}

impl<T, U> Service<TaskRequest<T>> for TaskHandler<T, U> {
    type Response = TaskResponse<U>;
    type Error = TaskError;
    type Future = TaskFuture<U>;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    #[inline]
    fn call(&mut self, req: TaskRequest<T>) -> Self::Future {
        self.inner.call(req).into()
    }
}

impl<T, U> Load for TaskHandler<T, U> {
    type Metric = TaskMetrics;

    #[inline]
    fn load(&self) -> Self::Metric {
        self.metrics.deref().clone()
    }
}

/// `tower::`[`Layer`] that produces a [`TaskHandler`] services.
pub struct TaskHandlerLayer<Req, T, U = T> {
    inner: PhantomData<(Req, T, U)>,
}

impl<Req, T, U> TaskHandlerLayer<Req, T, U> {
    /// Returns a new [`TaskHandlerLayer`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<Req, T, U> Default for TaskHandlerLayer<Req, T, U> {
    fn default() -> Self {
        Self { inner: PhantomData }
    }
}

impl<S, Req, T, U> Layer<S> for TaskHandlerLayer<Req, T, U>
where
    T: 'static,
    U: 'static,
    S: Service<Req> + Clone + Send + 'static,
    Req: From<TaskRequest<T>> + 'static,
    S::Response: Into<TaskResponse<U>> + 'static,
    S::Error: Into<TaskError> + 'static,
    S::Future: Send + 'static,
{
    type Service = TaskHandler<T, U>;

    #[inline]
    fn layer(&self, inner: S) -> Self::Service {
        TaskHandler::new(inner)
    }
}

#[cfg(test)]
mod test {
    use tower::{service_fn, ServiceBuilder};

    use crate::context::{TaskError, TaskRequest, TaskResponse};
    use crate::handler::{TaskHandler, TaskHandlerLayer};
    use crate::Result;

    async fn handle(request: TaskRequest<u32>) -> Result<TaskResponse<u32>, TaskError> {
        Ok(TaskResponse::new(request.into_inner()))
    }

    #[test]
    fn manual_compose() -> Result<()> {
        let inner = service_fn(handle);
        let _service = TaskHandler::new(inner);
        Ok(())
    }

    #[test]
    fn service_builder() -> Result<()> {
        let _service = ServiceBuilder::new()
            .layer(TaskHandlerLayer::new())
            .service(service_fn(handle));
        Ok(())
    }
}
