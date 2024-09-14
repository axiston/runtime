/// Unrecoverable failure duration [`TaskHandler`] execution.
///
/// [`TaskHandler`]: crate::handler::TaskHandler
#[derive(Debug, thiserror::Error)]
#[must_use = "errors do nothing unless you use them"]
#[error("failure during `TaskHandler` execution")]
pub struct TaskError {
    // TODO: Implement From<Box<dyn Error>>.

    // name: String,
    // message: String,
    // explain: String,
}

impl TaskError {}

#[cfg(test)]
mod test {}
