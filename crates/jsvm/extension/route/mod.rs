//! Runtime [`extension`] for the [`routing`] events.
//!
//! [`routing`]: axiston_runtime_task::registry::Registry

mod datatype;
mod internal;
mod ops;

use deno_core::extension;
use crate::extension::route::ops::{
    op_register_service, op_register_trigger, op_register_action
};

extension!(
    axis_routing,
    ops = [op_register_service, op_register_trigger, op_register_action],
    esm_entry_point = "ext:extension/route/ops.js",
    esm = [ dir "extension/route", "ops.js" ],
);

/// Unrecoverable failure during routing ops.
///
/// Includes all error types that may occur.
#[derive(Debug, thiserror::Error)]
#[must_use = "errors do nothing unless you use them"]
pub enum Error {
}

/// Specialized [`Result`] alias for [`Error`].
///
/// [`Result`]: std::result::Result
pub type Result<T, E = Error> = std::result::Result<T, E>;
