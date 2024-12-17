use std::error::Error;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Unrecoverable failure duration [`TaskHandler`] execution.
///
/// [`TaskHandler`]: crate::handler::TaskHandler
#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
#[error("internal handler error")]
#[must_use = "errors do nothing unless you use them"]
pub struct TaskError {
    pub(crate) values: Value,
}

impl TaskError {
    /// Returns a new [`TaskError`].
    #[inline]
    pub fn new<E>(values: Value) -> Self
    where
        E: Error + 'static,
    {
        Self { values }
    }

    /// Returns a new [`TaskErrorBuilder`].
    #[inline]
    pub fn builder() -> TaskErrorBuilder {
        TaskErrorBuilder::new()
    }
}

/// Specialized [`Result`] alias for the [`TaskError`] type.
pub type TaskResult<T, E = TaskError> = Result<T, E>;

/// [`TaskHandler`] service error builder.
///
/// [`TaskHandler`]: crate::handler::TaskHandler
#[derive(Debug, Clone)]
#[must_use = "errors do nothing unless you serialize them"]
pub struct TaskErrorBuilder {
    values: Option<Value>,
}

impl TaskErrorBuilder {
    /// Returns a new [`TaskErrorBuilder`].
    #[inline]
    pub fn new() -> Self {
        Self { values: None }
    }

    /// Overrides the default value of [`TaskError`]`::values`.
    #[inline]
    pub fn with_values(mut self, values: Value) -> Self {
        self.values = Some(values);
        self
    }

    /// Returns a new [`TaskError`].
    pub fn build(self) -> TaskError {
        TaskError {
            values: self.values.unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod test {
    use serde_json::Value;

    use crate::context::TaskError;

    #[test]
    fn build_empty_error() -> crate::Result<()> {
        let _error = TaskError::builder().with_values(Value::default()).build();

        Ok(())
    }
}
