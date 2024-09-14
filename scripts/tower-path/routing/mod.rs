//! TODO.
//!

use std::collections::HashMap;
use std::fmt;
use std::marker::PhantomData;
use std::task::{Context, Poll};

use tower::Service;

use crate::routing::container::RouterContainer;
use crate::routing::index::{DefaultIx, RouterIndex};

pub mod container;
pub mod index;

/// TODO.
#[derive(Clone)]
#[must_use = "routers do nothing unless you use them"]
pub struct Router<S, I = DefaultIx, Contain = HashMap<I, S>> {
    index: PhantomData<I>,
    service: PhantomData<S>,
    inner: Contain,
}

impl<S, I, Contain> Router<S, I, Contain> {
    /// Returns a new [`Router`].
    #[inline]
    pub fn new(inner: Contain) -> Self {
        Self {
            index: PhantomData,
            service: PhantomData,
            inner,
        }
    }

    /// Returns the underlying route container.
    #[inline]
    pub fn into_inner(self) -> Contain {
        self.inner
    }
}

impl<S, I, Contain> Router<S, I, Contain>
where
    I: RouterIndex,
    Contain: RouterContainer<I, S>,
{
    /// Inserts an index-handler pair into the [`Router`].
    #[inline]
    pub fn route(mut self, ix: I, route: S) -> Self {
        let _ = self.inner.route(ix, route);
        self
    }

    /// Removes an index from the [`Router`].
    #[inline]
    pub fn forget(mut self, ix: I) -> Self {
        let _ = self.inner.forget(ix);
        self
    }
}

impl<S, I, Contain> Default for Router<S, I, Contain>
where
    Contain: Default,
{
    #[inline]
    fn default() -> Self {

        Self::new(Contain::default())

    }
}

impl<S, I, Contain> fmt::Debug for Router<S, I, Contain> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Router").finish_non_exhaustive()
    }
}

impl<S, I, Contain, Req> Service<Req> for Router<S, I, Contain>
where
    S: Service<Req>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        todo!()
    }

    #[inline]
    fn call(&mut self, req: Req) -> Self::Future {
        todo!()
    }
}

#[cfg(test)]
mod test {}
