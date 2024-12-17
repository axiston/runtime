use std::fmt;

use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::routing::layers::Layers;

/// Serializable [`TaskHandler`] service request.
///
/// [`TaskHandler`]: crate::handler::TaskHandler
#[derive(Clone, Serialize, Deserialize, Deref, DerefMut)]
#[must_use = "requests do nothing unless you serialize them"]
pub struct TaskRequest<T = ()> {
    #[deref]
    #[deref_mut]
    inner: T,

    pub(crate) inputs: Value,
    pub(crate) secrets: Value,
    pub(crate) layers: Layers,
}

impl<T> TaskRequest<T> {
    /// Returns a new [`TaskRequest`].
    #[inline]
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            inputs: Value::default(),
            secrets: Value::default(),
            layers: Layers::new(),
        }
    }

    /// Returns a new [`TaskRequestBuilder`].
    #[inline]
    pub fn builder(inner: T) -> TaskRequestBuilder<T> {
        TaskRequestBuilder::new(inner)
    }

    /// Returns the inner data.
    #[inline]
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> fmt::Debug for TaskRequest<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TaskRequest")
            .field("inputs", &self.inputs)
            .field("secrets", &self.secrets)
            .finish_non_exhaustive()
    }
}

/// [`TaskHandler`] service request builder.
///
/// [`TaskHandler`]: crate::handler::TaskHandler
#[derive(Debug, Clone)]
#[must_use = "requests do nothing unless you serialize them"]
pub struct TaskRequestBuilder<T> {
    inner: T,
    inputs: Option<Value>,
    secrets: Option<Value>,
    layers: Option<Layers>,
}

impl<T> TaskRequestBuilder<T> {
    /// Returns a new [`TaskRequestBuilder`].
    #[inline]
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            inputs: None,
            secrets: None,
            layers: None,
        }
    }

    /// Overrides the default value of [`TaskRequest`]`::inputs`.
    #[inline]
    pub fn with_inputs(mut self, json: Value) -> Self {
        self.inputs = Some(json);
        self
    }

    /// Overrides the default value of [`TaskRequest`]`::secrets`.
    #[inline]
    pub fn with_secrets(mut self, json: Value) -> Self {
        self.secrets = Some(json);
        self
    }

    /// Overrides the default value of [`TaskRequest`]`::layers`.
    #[inline]
    pub fn with_layers(mut self, layers: Layers) -> Self {
        self.layers = Some(layers);
        self
    }

    /// Returns a new [`TaskRequest`].
    pub fn build(self) -> TaskRequest<T> {
        TaskRequest {
            inner: self.inner,
            inputs: self.inputs.unwrap_or_default(),
            secrets: self.secrets.unwrap_or_default(),
            layers: self.layers.unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod test {
    use serde_json::Value;

    use crate::context::TaskRequest;
    use crate::routing::layers::Layers;
    use crate::Result;

    #[test]
    fn build_empty_request() -> Result<()> {
        let _request = TaskRequest::builder(5)
            .with_inputs(Value::default())
            .with_secrets(Value::default())
            .with_layers(Layers::new())
            .build();
        Ok(())
    }
}
