#![forbid(unsafe_code)]
#![doc = include_str!("./README.md")]

//! TODO.

use std::collections::HashMap;

use runtime_task::routing::{Index, Router};
use runtime_type::datatype::condition::{ConditionRequestData, ConditionResponseData};
use runtime_type::datatype::operation::{OperationRequestData, OperationResponseData};
use runtime_type::manifest::condition::ConditionManifest;
use runtime_type::manifest::operation::OperationManifest;

#[derive(Debug, Default, Clone)]
pub struct AppRouter {
    // groups: HashMap<Index>
    conditions: Router<ConditionManifest, ConditionRequestData, ConditionResponseData>,
    operations: Router<OperationManifest, OperationRequestData, OperationResponseData>,
}

impl AppRouter {
    /// Returns an empty [`Router`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn retrieve_manifests() {}

    pub fn try_execute() {}
}
