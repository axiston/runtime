use std::fmt;

use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Deserializable [`TaskHandler`] service response.
///
/// [`TaskHandler`]: crate::handler::TaskHandler
#[derive(Clone, Serialize, Deserialize, Deref, DerefMut)]
#[must_use = "responses do nothing unless you serialize them"]
pub struct TaskResponse<T> {
    #[deref]
    #[deref_mut]
    inner: T,

    pub(crate) outputs: Value,
    pub(crate) metrics: Value,
}

impl<T> TaskResponse<T> {
    /// Returns a new [`TaskResponse`].
    #[inline]
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            outputs: Value::default(),
            metrics: Value::default(),
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
            .field("outputs", &self.outputs)
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
    outputs: Option<Value>,
    metrics: Option<Value>,
}

impl<T> TaskResponseBuilder<T> {
    /// Returns a new [`TaskResponseBuilder`].
    #[inline]
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            outputs: None,
            metrics: None,
        }
    }

    /// Overrides the default value of [`TaskResponse`]`::outputs`.
    #[inline]
    pub fn with_outputs(mut self, values: Value) -> Self {
        self.outputs = Some(values);
        self
    }

    /// Overrides the default value of [`TaskResponse`]`::metrics`.
    #[inline]
    pub fn with_metrics(mut self, values: Value) -> Self {
        self.metrics = Some(values);
        self
    }

    /// Returns a new [`TaskResponse`].
    pub fn build(self) -> TaskResponse<T> {
        TaskResponse {
            inner: self.inner,
            outputs: self.outputs.unwrap_or_default(),
            metrics: self.metrics.unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod test {
    use serde_json::Value;

    use crate::context::TaskResponse;
    use crate::Result;

    #[test]
    fn build_empty_response() -> Result<()> {
        let _response = TaskResponse::builder(5)
            .with_outputs(Value::default())
            .with_metrics(Value::default())
            .build();
        Ok(())
    }
}
