//! Runtime [`extension`] for the [`tracing`] events.
//!

use deno_core::extension;

use crate::extension::trace::ops::{
    op_tracing_debug, op_tracing_debug_fast, op_tracing_error, op_tracing_error_fast,
    op_tracing_info, op_tracing_info_fast, op_tracing_trace, op_tracing_trace_fast,
    op_tracing_warn, op_tracing_warn_fast,
};

mod datatype;
mod internal;
mod ops;

extension!(
    axis_tracing,
    ops = [
        op_tracing_trace_fast, op_tracing_trace, op_tracing_debug_fast, op_tracing_debug,
        op_tracing_info_fast, op_tracing_info, op_tracing_warn_fast, op_tracing_warn,
        op_tracing_error_fast, op_tracing_error],
    esm_entry_point = "ext:extension/trace/ops.js",
    esm = [ dir "extension/trace", "ops.js" ],
);

/// Unrecoverable failure during tracing ops.
///
/// Includes all error types that may occur.
#[derive(Debug, thiserror::Error)]
#[must_use = "errors do nothing unless you use them"]
pub enum Error {
    #[error("tracing target reused")]
    ReuseTarget,
}

/// Specialized [`Result`] alias for [`Error`].
///
/// [`Result`]: std::result::Result
pub type Result<T, E = Error> = std::result::Result<T, E>;
