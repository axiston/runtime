use std::fmt;
use std::ops::{Deref, DerefMut};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Deserializable [`TaskHandler`] service response.
///
/// [`TaskHandler`]: crate::handler::TaskHandler
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[must_use = "responses do nothing unless you serialize them"]
pub struct TaskResponse<T> {
    inner: T,
}

impl<T> TaskResponse<T> {
    /// Returns a new [`TaskResponse`].
    #[inline]
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    /// Returns a new [`TaskResponseBuilder`].
    #[inline]
    pub fn builder(inner: T) -> TaskResponseBuilder<T> {
        TaskResponseBuilder::new(inner)
    }

    /// Returns the inner data.
    #[inline]
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> Deref for TaskResponse<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for TaskResponse<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> fmt::Debug for TaskResponse<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TaskResponse").finish_non_exhaustive()
    }
}

/// [`TaskHandler`] service response builder.
///
/// [`TaskHandler`]: crate::handler::TaskHandler
#[derive(Debug, Default, Clone)]
#[must_use = "responses do nothing unless you serialize them"]
pub struct TaskResponseBuilder<T> {
    inner: T,
}

impl<T> TaskResponseBuilder<T> {
    /// Returns a new [`TaskResponseBuilder`].
    #[inline]
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    /// Returns a new [`TaskResponse`].
    pub fn build(self) -> TaskResponse<T> {
        TaskResponse { inner: self.inner }
    }
}
