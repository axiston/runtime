use std::time::Duration;

/// Applied `tower::`[`Layer`]s configuration.
///
/// [`Layer`]: tower::Layer
#[derive(Debug, Default, Clone)]
pub struct Layers {
    pub timeout_start_to_close: Duration,
    pub timeout_before_start: Duration,
    pub timeout_before_close: Duration,

    pub limit_concurrency_task: u32,
    pub limit_cpu_consumption: u32,
    pub limit_ram_consumption: u32,

    pub retry_initial_interval: Duration,
    pub retry_maximum_interval: Duration,
    pub retry_backoff_coefficient: u32,
    pub retry_maximum_attempts: u32,
}

impl Layers {
    /// Returns a new [`Layers`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}
