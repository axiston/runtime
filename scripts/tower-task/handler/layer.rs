use std::marker::PhantomData;

use tower::{Layer, Service};

use crate::context::{TaskError, TaskRequest, TaskResponse};
use crate::handler::TaskHandler;

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
    use crate::handler::TaskHandlerLayer;
    use crate::Result;

    #[test]
    fn layer() -> Result<()> {
        let _ = TaskHandlerLayer::<u32, u32>::new();
        Ok(())
    }
}
