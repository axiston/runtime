//! [`WithData`] and [`WithDataLayer`].

use std::fmt;
use std::ops::{Deref, DerefMut};
use std::task::{Context, Poll};

use tower::Service;

pub use crate::service::layer::WithDataLayer;
mod layer;

/// Simple `tower::`[`Service`] for attaching additional metadata.
#[must_use = "services do nothing unless you `.poll_ready` or `.call` them"]
pub struct WithData<S, M> {
    inner: S,
    data: M,
}

impl<S, M> WithData<S, M> {
    /// Returns a new [`WithData`].
    #[inline]
    pub fn new(inner: S, data: M) -> Self {
        Self { inner, data }
    }

    /// Returns the underlying parts.
    #[inline]
    pub fn into_inner(self) -> (S, M) {
        (self.inner, self.data)
    }
}

impl<S, M> Deref for WithData<S, M> {
    type Target = M;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<S, M> DerefMut for WithData<S, M> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<S, M> fmt::Debug for WithData<S, M>
where
    M: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WithData")
            .field("data", &self.data)
            .finish_non_exhaustive()
    }
}

impl<S, M, Req> Service<Req> for WithData<S, M>
where
    S: Service<Req>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    #[inline]
    fn call(&mut self, req: Req) -> Self::Future {
        self.inner.call(req)
    }
}

