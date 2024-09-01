//! [`TaskHandler`] service, its future and metrics.

use std::fmt;
use std::task::{Context, Poll};

use tower::load::Load;
use tower::util::BoxCloneService;
use tower::{Service, ServiceBuilder};

use crate::context::{TaskError, TaskRequest, TaskResponse};
use crate::handler::future::TaskFuture;
pub use crate::handler::layer::TaskHandlerLayer;
use crate::handler::metric::TaskMetric;
use crate::handler::native::NativeTask;

pub mod future;
mod layer;
pub mod metric;
pub mod native;

/// Unified `tower::`[`Service`] for executing [`tasks`].
///
/// Opaque [`BoxCloneService`]<[`TaskRequest`], [`TaskResponse`], [`TaskError`]>.
///
/// [`tasks`]: crate::context
#[must_use = "services do nothing unless you `.poll_ready` or `.call` them"]
pub struct TaskHandler<M, T, U> {
    manifest: M,
    inner: BoxCloneService<TaskRequest<T>, TaskResponse<U>, TaskError>,
}

impl<M, T, U> TaskHandler<M, T, U> {
    /// Returns a new [`TaskHandler`].
    #[inline]
    pub fn new<S, Req>(manifest: M, inner: S) -> Self
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
            manifest,
            inner: BoxCloneService::new(inner),
        }
    }

    /// Returns a new [`NativeTask`] wrapped in [`TaskHandler`].
    pub fn native(manifest: M) -> Self
    where
        T: Send + 'static,
        U: Send + 'static,
    {
        Self::new(manifest, NativeTask::new())
    }

    /// Returns a reference to the manifest data.
    #[inline]
    pub fn manifest_ref(&self) -> &M {
        &self.manifest
    }

    /// Returns a mutable reference to the manifest data.
    #[inline]
    pub fn manifest_mut(&mut self) -> &mut M {
        &mut self.manifest
    }

    /// Maps an `TaskHandler<M, T, U>` to `TaskHandler<M, T2, U2>` by applying a function to a contained service.
    pub fn map<T2, U2, F>(self, f: F) -> TaskHandler<M, T2, U2>
    where
        F: FnOnce(
            BoxCloneService<TaskRequest<T>, TaskResponse<U>, TaskError>,
        ) -> BoxCloneService<TaskRequest<T2>, TaskResponse<U2>, TaskError>,
    {
        TaskHandler {
            manifest: self.manifest,
            inner: f(self.inner),
        }
    }

    /// Estimates the service's current load.
    pub fn metrics(&self) -> TaskMetric {
        TaskMetric::new()
    }
}

impl<M, T, U> Clone for TaskHandler<M, T, U>
where
    M: Clone,
{
    fn clone(&self) -> Self {
        Self {
            manifest: self.manifest.clone(),
            inner: self.inner.clone(),
        }
    }
}

impl<M, T, U> fmt::Debug for TaskHandler<M, T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TaskHandler").finish_non_exhaustive()
    }
}

impl<M, T, U> Service<TaskRequest<T>> for TaskHandler<M, T, U> {
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

impl<M, T, U> Load for TaskHandler<M, T, U> {
    type Metric = TaskMetric;

    #[inline]
    fn load(&self) -> Self::Metric {
        self.metrics()
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
    fn service() -> Result<()> {
        let _ = TaskHandler::new((), service_fn(handle));
        Ok(())
    }

    #[test]
    fn layer() -> Result<()> {
        let _ = ServiceBuilder::new()
            .layer(TaskHandlerLayer::new(()))
            .service(service_fn(handle));
        Ok(())
    }
}
