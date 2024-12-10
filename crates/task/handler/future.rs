//! Futures types for [`TaskHandler`]s.
//!
//! [`TaskHandler`]: crate::handler::TaskHandler

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::future::BoxFuture;
use futures::FutureExt;
use pin_project_lite::pin_project;

use crate::context::{TaskError, TaskResponse};

pin_project! {
    /// Opaque [`Future`] return type for [`TaskHandler::call`].
    ///
    /// Opaque `futures::`[`BoxFuture`].
    ///
    /// [`TaskHandler::call`]: crate::context::TaskHandler
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct TaskFuture<U> {
        #[pin] fut: BoxFuture<'static, Result<TaskResponse<U>, TaskError>>,
    }
}

impl<U> TaskFuture<U> {
    /// Returns a new [`TaskFuture`].
    #[inline]
    pub fn new<F>(fut: F) -> Self
    where
        F: Future<Output = Result<TaskResponse<U>, TaskError>>,
        F: Sized + Send + 'static,
    {
        Self { fut: fut.boxed() }
    }
}

impl<U> From<BoxFuture<'static, Result<TaskResponse<U>, TaskError>>> for TaskFuture<U> {
    #[inline]
    fn from(fut: BoxFuture<'static, Result<TaskResponse<U>, TaskError>>) -> Self {
        Self { fut }
    }
}

impl<U> Future for TaskFuture<U> {
    type Output = Result<TaskResponse<U>, TaskError>;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        this.fut.poll(cx)
    }
}

#[cfg(test)]
mod test {
    use crate::context::TaskResponse;
    use crate::handler::future::TaskFuture;
    use crate::Result;

    #[test]
    fn future_from_block() -> Result<()> {
        let fut = async move { Ok(TaskResponse::new(5)) };
        let _fut = TaskFuture::new(fut);

        Ok(())
    }
}
