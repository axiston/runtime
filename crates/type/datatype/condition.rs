//! [`ConditionRequestData`] and [`ConditionResponseData`] types.
//!

use serde::{Deserialize, Serialize};

use crate::datatype::{RequestData, ResponseData};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConditionRequestData {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConditionResponseData {}
