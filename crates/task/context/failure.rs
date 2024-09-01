use std::error::Error;

/// Unrecoverable failure duration [`TaskHandler`] execution.
///
/// [`TaskHandler`]: crate::handler::TaskHandler
#[derive(Debug, thiserror::Error)]
#[must_use = "errors do nothing unless you use them"]
#[error("failure during `TaskHandler` execution")]
pub struct TaskError {
    // name: String,
    // message: String,
    // explain: String,
}

impl TaskError {
    /// Returns a new [`TaskError`].
    #[inline]
    pub fn new<T>(error: T) -> Self {
        Self {}
    }
}

#[cfg(test)]
mod test {
    use crate::context::TaskError;
    use crate::Result;

    #[test]
    fn build() -> Result<()> {
        let _ = TaskError::new(());
        Ok(())
    }
}
