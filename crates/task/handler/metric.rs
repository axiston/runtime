//! [`Load`] metric types for [`TaskHandler`]s.
//!
//! [`Load`]: tower::load::Load
//! [`TaskHandler`]: crate::handler::TaskHandler

use serde::{Deserialize, Serialize};

/// `tower::load::`[`Load`] metrics for [`TaskHandler`]s.
///
/// [`Load`]: tower::load::Load
/// [`TaskHandler`]: crate::handler::TaskHandler
#[derive(Debug, Default, Clone, PartialOrd, PartialEq, Serialize, Deserialize)]
#[must_use = "metrics do nothing unless you serialize them"]
pub struct TaskMetrics {}

impl TaskMetrics {
    /// Returns a new [`TaskMetrics`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod test {
    use crate::handler::metric::TaskMetrics;
    use crate::Result;

    #[test]
    fn from_default() -> Result<()> {
        let _metrics = TaskMetrics::new();
        Ok(())
    }
}
