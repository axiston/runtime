//! [`Load`] metric types for [`TaskHandler`]s.
//!
//! [`Load`]: tower::load::Load
//! [`TaskHandler`]: crate::handler::TaskHandler

use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

/// Reference-counting utility for [`TaskMetrics`].
#[derive(Debug, Default, Clone)]
#[must_use = "metrics do nothing unless you serialize them"]
pub struct LockTaskMetrics {
    inner: Arc<Mutex<TaskMetrics>>,
}

impl LockTaskMetrics {
    /// Returns a new [`LockTaskMetrics`].
    #[inline]
    pub fn new(metrics: TaskMetrics) -> Self {
        Self {
            inner: Arc::new(Mutex::new(metrics)),
        }
    }

    /// Returns a new [`TaskMetrics`].
    pub fn snapshot(&self) -> TaskMetrics {
        TaskMetrics {}
    }
}

/// `tower::load::`[`Load`] metrics for [`TaskHandler`]s.
///
/// [`Load`]: tower::load::Load
/// [`TaskHandler`]: crate::handler::TaskHandler
#[derive(Debug, Default, Clone, PartialOrd, PartialEq, Serialize, Deserialize)]
#[must_use = "metrics do nothing unless you serialize them"]
pub struct TaskMetrics {
    // TODO: pub average_waiting_time: Duration,
    // TODO: pub average_recent_waiting_time: Duration,
    // TODO: pub average_running_time: Duration,
    // TODO: pub average_recent_running_time: Duration,
    // TODO: pub total_success_runs: u32,
    // TODO: pub total_failure_runs: u32,
}

impl TaskMetrics {
    /// Returns a new [`TaskMetrics`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod test {
    use crate::handler::metric::{LockTaskMetrics, TaskMetrics};
    use crate::Result;

    #[test]
    fn metrics_lock() -> Result<()> {
        let _metrics = LockTaskMetrics::default();
        Ok(())
    }

    #[test]
    fn default_metrics() -> Result<()> {
        let _metrics = TaskMetrics::new();
        Ok(())
    }
}
