use std::fmt;
use std::ops::{Deref, DerefMut};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::routing::Index;

/// Serializable [`TaskHandler`] service request.
///
/// [`TaskHandler`]: crate::handler::TaskHandler
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[must_use = "requests do nothing unless you serialize them"]
pub struct TaskRequest<T> {
    index: Index,
    inner: T,
}

impl<T> TaskRequest<T> {
    /// Returns a new [`TaskRequest`].
    #[inline]
    pub fn new(index: Index, inner: T) -> Self {
        Self { index, inner }
    }

    /// Returns a new [`TaskRequestBuilder`].
    #[inline]
    pub fn builder(index: Index, inner: T) -> TaskRequestBuilder<T> {
        TaskRequestBuilder::new(index, inner)
    }

    /// Returns the reference to the [`Index`].
    #[inline]
    pub fn index(&self) -> &Index {
        &self.index
    }

    /// Returns the inner data.
    #[inline]
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> Deref for TaskRequest<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for TaskRequest<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> fmt::Debug for TaskRequest<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TaskRequest").finish_non_exhaustive()
    }
}

/// [`TaskHandler`] service request builder.
///
/// [`TaskHandler`]: crate::handler::TaskHandler
#[derive(Debug, Clone)]
#[must_use = "requests do nothing unless you serialize them"]
pub struct TaskRequestBuilder<T> {
    index: Index,
    inner: T,
}

impl<T> TaskRequestBuilder<T> {
    /// Returns a new [`TaskRequestBuilder`].
    #[inline]
    pub fn new(index: Index, inner: T) -> Self {
        Self { index, inner }
    }

    /// Returns a new [`TaskRequest`].
    pub fn build(self) -> TaskRequest<T> {
        TaskRequest {
            index: self.index,
            inner: self.inner,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::context::TaskRequest;
    use crate::routing::Index;
    use crate::Result;

    #[test]
    fn build() -> Result<()> {
        let index = Index::new("request-id");
        let _ = TaskRequest::builder(index, 5).build();
        Ok(())
    }
}
