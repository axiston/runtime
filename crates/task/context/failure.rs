use std::error::Error;

use serde::{Deserialize, Serialize};
use thiserror::Error;

// TODO: wrap box error instead

/// Unrecoverable failure duration [`TaskHandler`] execution.
///
/// [`TaskHandler`]: crate::handler::TaskHandler
#[derive(Debug, Error, Serialize, Deserialize)]
#[must_use = "errors do nothing unless you use them"]
#[error("failure during `TaskHandler` execution")]
pub struct TaskError {
    name: String,
    message: String,
}

impl TaskError {
    /// Returns a new [`TaskError`].
    pub fn new() -> Self {
        Self {
            name: "".to_owned(),
            message: "".to_owned(),
        }
    }

    /// Returns the underlying error's name.
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the underlying error's message.
    #[inline]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl From<Box<dyn Error>> for TaskError {
    fn from(value: Box<dyn Error>) -> Self {
        todo!()
    }
}

/// Specialized [`Result`] alias for the [`TaskError`] type.
pub type TaskResult<T, E = TaskError> = Result<T, E>;

#[cfg(test)]
mod test {
    use crate::context::TaskError;
    use crate::Result;

    #[test]
    fn instance() -> Result<()> {
        let _ = TaskError::new();
        Ok(())
    }
}
