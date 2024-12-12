use std::error::Error;

/// Unrecoverable failure duration [`TaskHandler`] execution.
///
/// [`TaskHandler`]: crate::handler::TaskHandler
#[derive(Debug, thiserror::Error)]
#[error("internal handler error")]
#[must_use = "errors do nothing unless you use them"]
pub struct TaskError {
    inner: Box<dyn Error>,
}

impl TaskError {
    /// Returns a new [`TaskError`].
    #[inline]
    pub fn new<E>(inner: E) -> Self
    where
        E: Error + 'static,
    {
        Self {
            inner: Box::new(inner),
        }
    }
}

/// Specialized [`Result`] alias for the [`TaskError`] type.
pub type TaskResult<T, E = TaskError> = Result<T, E>;
