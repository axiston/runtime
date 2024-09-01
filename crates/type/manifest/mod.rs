//! Associated [`Manifest`] types.
//!

use serde::{Deserialize, Serialize};

use crate::manifest::condition::ConditionManifest;
use crate::manifest::operation::OperationManifest;

pub mod condition;
pub mod operation;

/// Associated metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Manifest {
    Condition(ConditionManifest),
    Operation(OperationManifest),
}

impl From<ConditionManifest> for Manifest {
    #[inline]
    fn from(value: ConditionManifest) -> Self {
        Self::Condition(value)
    }
}

impl From<OperationManifest> for Manifest {
    #[inline]
    fn from(value: OperationManifest) -> Self {
        Self::Operation(value)
    }
}
