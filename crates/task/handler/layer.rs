use std::marker::PhantomData;

use tower::{Layer, Service};

use crate::context::{TaskError, TaskRequest, TaskResponse};
use crate::handler::TaskHandler;

/// `tower::`[`Layer`] that produces a [`TaskHandler`] services.
pub struct TaskHandlerLayer<M, Req, T, U = T> {
    manifest: M,
    inner: PhantomData<Req>,
    request: PhantomData<T>,
    response: PhantomData<U>,
}

impl<M, Req, T, U> TaskHandlerLayer<M, Req, T, U> {
    /// Returns a new [`TaskHandlerLayer`].
    #[inline]
    pub fn new(manifest: M) -> Self {
        Self {
            manifest,
            inner: PhantomData,
            request: PhantomData,
            response: PhantomData,
        }
    }
}

impl<M, Req, T, U> Default for TaskHandlerLayer<M, Req, T, U>
where
    M: Default,
{
    #[inline]
    fn default() -> Self {
        Self {
            manifest: M::default(),
            inner: PhantomData,
            request: PhantomData,
            response: PhantomData,
        }
    }
}

impl<S, M, Req, T, U> Layer<S> for TaskHandlerLayer<M, Req, T, U>
where
    M: Clone,
    T: 'static,
    U: 'static,
    S: Service<Req> + Clone + Send + 'static,
    Req: From<TaskRequest<T>> + 'static,
    S::Response: Into<TaskResponse<U>> + 'static,
    S::Error: Into<TaskError> + 'static,
    S::Future: Send + 'static,
{
    type Service = TaskHandler<M, T, U>;

    #[inline]
    fn layer(&self, inner: S) -> Self::Service {
        TaskHandler::new(self.manifest.clone(), inner)
    }
}

#[cfg(test)]
mod test {
    use crate::handler::TaskHandlerLayer;
    use crate::Result;

    #[test]
    fn layer() -> Result<()> {
        let _ = TaskHandlerLayer::<(), u32, u32>::new(());
        Ok(())
    }
}
