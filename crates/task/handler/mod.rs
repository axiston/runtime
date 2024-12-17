//! [`TaskHandler`], [`TaskHandlerLayer`], its future and metrics.

use std::fmt;
use std::marker::PhantomData;
use std::task::{Context, Poll};

use tower::load::Load;
use tower::util::BoxCloneSyncService;
use tower::{Layer, Service, ServiceBuilder};

use crate::context::{TaskError, TaskRequest, TaskResponse};
use crate::handler::future::TaskFuture;
use crate::handler::metric::{LockTaskMetrics, TaskMetrics};

pub mod future;
pub mod metric;

/// Unified `tower::`[`Service`] for executing tasks.
///
/// Opaque [`BoxCloneSyncService`]<[`TaskRequest`], [`TaskResponse`], [`TaskError`]>.
#[must_use = "services do nothing unless you `.poll_ready` or `.call` them"]
pub struct TaskHandler<T, U> {
    inner: BoxCloneSyncService<TaskRequest<T>, TaskResponse<U>, TaskError>,
    metrics: LockTaskMetrics,
}

impl<T, U> TaskHandler<T, U> {
    /// Returns a new [`TaskHandler`].
    pub fn new<S, Req>(inner: S) -> Self
    where
        T: 'static,
        U: 'static,
        S: Service<Req> + Clone + Send + Sync + 'static,
        Req: From<TaskRequest<T>> + 'static,
        S::Response: Into<TaskResponse<U>> + 'static,
        S::Error: Into<TaskError> + 'static,
        S::Future: Send + 'static,
    {
        Self::with_metrics(inner, LockTaskMetrics::default())
    }

    /// Returns a new [`TaskHandler`] with provided [`LockTaskMetrics`].
    ///
    /// Allows to share [`LockTaskMetrics`] and the inner [`TaskMetrics`].
    pub fn with_metrics<S, Req>(inner: S, metrics: LockTaskMetrics) -> Self
    where
        T: 'static,
        U: 'static,
        S: Service<Req> + Clone + Send + Sync + 'static,
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
            inner: BoxCloneSyncService::new(inner),
            metrics,
        }
    }

    /// Maps a `TaskHandler<T, U>` to `TaskHandler<T2, U2>` by applying a function to a contained service.
    pub fn map<T2, U2, F>(self, f: F) -> TaskHandler<T2, U2>
    where
        F: FnOnce(
            BoxCloneSyncService<TaskRequest<T>, TaskResponse<U>, TaskError>,
        ) -> BoxCloneSyncService<TaskRequest<T2>, TaskResponse<U2>, TaskError>,
    {
        TaskHandler {
            inner: f(self.inner),
            metrics: self.metrics,
        }
    }

    /// Returns a new [`TaskMetrics`].
    #[inline]
    pub fn snapshot(&self) -> TaskMetrics {
        self.metrics.snapshot()
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

impl<T, U> Service<TaskRequest<T>> for TaskHandler<T, U>
where
    T: 'static,
    U: 'static,
{
    type Response = TaskResponse<U>;
    type Error = TaskError;
    type Future = TaskFuture<U>;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    #[inline]
    fn call(&mut self, req: TaskRequest<T>) -> Self::Future {
        TaskFuture::with_metrics(self.inner.call(req), self.metrics.clone())
    }
}

impl<T, U> Load for TaskHandler<T, U> {
    type Metric = TaskMetrics;

    #[inline]
    fn load(&self) -> Self::Metric {
        self.metrics.snapshot()
    }
}

/// `tower::`[`Layer`] that produces a [`TaskHandler`] services.
pub struct TaskHandlerLayer<Req, T, U = T> {
    metrics: LockTaskMetrics,
    inner: PhantomData<(Req, T, U)>,
}

impl<Req, T, U> TaskHandlerLayer<Req, T, U> {
    /// Returns a new [`TaskHandlerLayer`].
    #[inline]
    pub fn new(metrics: LockTaskMetrics) -> Self {
        Self {
            metrics,
            inner: PhantomData,
        }
    }
}

impl<Req, T, U> Default for TaskHandlerLayer<Req, T, U> {
    #[inline]
    fn default() -> Self {
        Self {
            metrics: LockTaskMetrics::default(),
            inner: PhantomData,
        }
    }
}

impl<S, Req, T, U> Layer<S> for TaskHandlerLayer<Req, T, U>
where
    T: 'static,
    U: 'static,
    S: Service<Req> + Clone + Send + Sync + 'static,
    Req: From<TaskRequest<T>> + 'static,
    S::Response: Into<TaskResponse<U>> + 'static,
    S::Error: Into<TaskError> + 'static,
    S::Future: Send + 'static,
{
    type Service = TaskHandler<T, U>;

    #[inline]
    fn layer(&self, inner: S) -> Self::Service {
        TaskHandler::with_metrics(inner, self.metrics.clone())
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
    fn service_compose() -> Result<()> {
        let inner = service_fn(handle);
        let _service = TaskHandler::new(inner);
        Ok(())
    }

    #[test]
    fn service_builder() -> Result<()> {
        let _service = ServiceBuilder::new()
            .layer(TaskHandlerLayer::default())
            .service(service_fn(handle));
        Ok(())
    }
}
