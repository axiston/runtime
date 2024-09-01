//! [`RequestData`] and [`ResponseData`] types.
//!

use serde::{Deserialize, Serialize};

use crate::datatype::condition::{ConditionRequestData, ConditionResponseData};
use crate::datatype::operation::{OperationRequestData, OperationResponseData};

pub mod condition;
pub mod operation;

#[derive(Debug, Serialize, Deserialize)]
pub enum RequestData {
    Condition(ConditionRequestData),
    Operation(OperationRequestData),
}

impl RequestData {
    /// Returns a new [`RequestData`].
    #[inline]
    pub fn new(data: impl Into<RequestData>) -> Self {
        data.into()
    }
}

impl From<ConditionRequestData> for RequestData {
    #[inline]
    fn from(value: ConditionRequestData) -> Self {
        Self::Condition(value)
    }
}

impl From<OperationRequestData> for RequestData {
    #[inline]
    fn from(value: OperationRequestData) -> Self {
        Self::Operation(value)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseData {
    Condition(ConditionResponseData),
    Operation(OperationResponseData),
}

impl ResponseData {
    /// Returns a new [`ResponseData`].
    #[inline]
    pub fn new(data: impl Into<ResponseData>) -> Self {
        data.into()
    }
}

impl From<ConditionResponseData> for ResponseData {
    #[inline]
    fn from(value: ConditionResponseData) -> Self {
        Self::Condition(value)
    }
}

impl From<OperationResponseData> for ResponseData {
    #[inline]
    fn from(value: OperationResponseData) -> Self {
        Self::Operation(value)
    }
}
