use std::collections::HashMap;
use std::fmt;

use derive_more::{Deref, DerefMut, From};
use serde::{Deserialize, Serialize};

/// TODO.
#[derive(Debug, Default, Clone, Serialize, Deserialize, From)]
#[must_use = "responses do nothing unless you serialize them"]
pub struct Metrics {
    inner: HashMap<String, String>,
}

impl Metrics {
    /// Returns an empty [`Metrics`] store.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

/// Deserializable [`TaskHandler`] service response.
///
/// [`TaskHandler`]: crate::handler::TaskHandler
#[derive(Clone, Serialize, Deserialize, Deref, DerefMut)]
#[must_use = "responses do nothing unless you serialize them"]
pub struct TaskResponse<T> {
    #[deref]
    #[deref_mut]
    inner: T,

    metrics: Metrics,
}

impl<T> TaskResponse<T> {
    /// Returns a new [`TaskResponse`].
    #[inline]
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            metrics: Metrics::default(),
        }
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

impl<T> fmt::Debug for TaskResponse<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TaskResponse")
            .field("metrics", &self.metrics)
            .finish_non_exhaustive()
    }
}

/// [`TaskHandler`] service response builder.
///
/// [`TaskHandler`]: crate::handler::TaskHandler
#[derive(Debug, Default, Clone)]
#[must_use = "responses do nothing unless you serialize them"]
pub struct TaskResponseBuilder<T> {
    inner: T,
    metrics: Option<Metrics>,
}

impl<T> TaskResponseBuilder<T> {
    /// Returns a new [`TaskResponseBuilder`].
    #[inline]
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            metrics: None,
        }
    }

    /// Returns a new [`TaskResponse`].
    pub fn build(self) -> TaskResponse<T> {
        TaskResponse {
            inner: self.inner,
            metrics: self.metrics.unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::context::TaskResponse;
    use crate::Result;

    #[test]
    fn build() -> Result<()> {
        let _response = TaskResponse::builder(5).build();
        Ok(())
    }
}
