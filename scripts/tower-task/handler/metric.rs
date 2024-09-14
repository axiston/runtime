//! [`Load`] metric types for [`TaskHandler`]s.
//!
//! [`Load`]: tower::load::Load
//! [`TaskHandler`]: crate::handler::TaskHandler

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [`Load`] metric types for [`TaskHandler`]s.
///
/// [`Load`]: tower::load::Load
/// [`TaskHandler`]: crate::handler::TaskHandler
#[derive(Debug, Default, Clone, PartialOrd, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[must_use = "metrics do nothing unless you serialize them"]
pub struct TaskMetric {}

impl TaskMetric {
    /// Returns a new [`TaskMetric`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod test {
    use crate::handler::metric::TaskMetric;
    use crate::Result;

    #[test]
    fn build() -> Result<()> {
        let _ = TaskMetric::new();
        Ok(())
    }
}
