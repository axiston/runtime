use std::collections::HashMap;
use std::fmt;

use derive_more::{Deref, DerefMut, From};
use serde::{Deserialize, Serialize};

use crate::routing::Layers;

/// TODO.
#[derive(Debug, Default, Clone, Serialize, Deserialize, From)]
#[must_use = "requests do nothing unless you serialize them"]
pub struct Fields {
    inner: HashMap<String, String>,
}

impl Fields {
    /// Returns an empty [`Fields`] store.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

/// TODO.
#[derive(Debug, Default, Clone, Serialize, Deserialize, From)]
#[must_use = "requests do nothing unless you serialize them"]
pub struct Secrets {
    inner: HashMap<String, String>,
}

impl Secrets {
    /// Returns an empty [`Secrets`] store.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

/// Serializable [`TaskHandler`] service request.
///
/// [`TaskHandler`]: crate::handler::TaskHandler
#[derive(Clone, Serialize, Deserialize, Deref, DerefMut)]
#[must_use = "requests do nothing unless you serialize them"]
pub struct TaskRequest<T = ()> {
    #[deref]
    #[deref_mut]
    inner: T,

    fields: Fields,
    secrets: Secrets,
    layers: Layers,
}

impl<T> TaskRequest<T> {
    /// Returns a new [`TaskRequest`].
    #[inline]
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            fields: Fields::new(),
            secrets: Secrets::new(),
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
            .field("fields", &self.fields)
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
    fields: Option<Fields>,
    secrets: Option<Secrets>,
    layers: Option<Layers>,
}

impl<T> TaskRequestBuilder<T> {
    /// Returns a new [`TaskRequestBuilder`].
    #[inline]
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            fields: None,
            secrets: None,
            layers: None,
        }
    }

    // TODO: Method to add a single field.
    // TODO: Method to add a single secret.

    /// Overrides the default value of [`TaskRequest`]`::fields`.
    #[inline]
    pub fn with_fields(mut self, fields: Fields) -> Self {
        self.fields = Some(fields);
        self
    }

    /// Overrides the default value of [`TaskRequest`]`::secrets`.
    #[inline]
    pub fn with_secrets(mut self, secrets: Secrets) -> Self {
        self.secrets = Some(secrets);
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
            fields: self.fields.unwrap_or_default(),
            secrets: self.secrets.unwrap_or_default(),
            layers: self.layers.unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::context::storages::{Fields, Secrets};
    use crate::context::TaskRequest;
    use crate::routing::Layers;
    use crate::Result;

    #[test]
    fn build() -> Result<()> {
        let _request = TaskRequest::builder(5)
            .with_fields(Fields::new())
            .with_secrets(Secrets::new())
            .with_layers(Layers::new())
            .build();
        Ok(())
    }
}
