//! TODO.
//!

use std::fmt;
use std::marker::PhantomData;
use std::task::{Context, Poll};

use tower::Service;

use crate::context::{TaskError, TaskRequest, TaskResponse};
use crate::handler::future::TaskFuture;

/// TODO.
#[must_use = "services do nothing unless you `.poll_ready` or `.call` them"]
pub struct NativeTask<T, U> {
    request: PhantomData<T>,
    response: PhantomData<U>,
}

impl<T, U> NativeTask<T, U> {
    /// Returns a new [`NativeTask`].
    #[inline]
    pub fn new() -> Self {
        Self {
            request: PhantomData,
            response: PhantomData,
        }
    }
}

impl<T, U> Clone for NativeTask<T, U> {
    fn clone(&self) -> Self {
        Self {
            request: PhantomData,
            response: PhantomData,
        }
    }
}

impl<T, U> fmt::Debug for NativeTask<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NativeTask").finish_non_exhaustive()
    }
}

impl<T, U> Service<TaskRequest<T>> for NativeTask<T, U> {
    type Response = TaskResponse<U>;
    type Error = TaskError;
    type Future = TaskFuture<U>;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        todo!()
    }

    #[inline]
    fn call(&mut self, req: TaskRequest<T>) -> Self::Future {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::Result;

    #[test]
    fn build() -> Result<()> {
        Ok(())
    }
}
