use deno_core::op2;
use tracing::Level;

use crate::extension::trace::datatype::TracingOptions;
use crate::extension::trace::internal::emit_op_tracing_event;
use crate::extension::trace::Result;

#[op2(fast)]
pub fn op_tracing_trace_fast(#[string] message: &str) -> Result<()> {
    emit_op_tracing_event(message, Level::TRACE, None)
}

#[op2(fast(op_tracing_trace_fast))]
pub fn op_tracing_trace(
    #[string] message: &str,
    #[serde] options: Option<TracingOptions>,
) -> Result<()> {
    emit_op_tracing_event(message, Level::TRACE, options)
}

#[op2(fast)]
pub fn op_tracing_debug_fast(#[string] message: &str) -> Result<()> {
    emit_op_tracing_event(message, Level::DEBUG, None)
}

#[op2(fast(op_tracing_debug_fast))]
pub fn op_tracing_debug(
    #[string] message: &str,
    #[serde] options: Option<TracingOptions>,
) -> Result<()> {
    emit_op_tracing_event(message, Level::DEBUG, options)
}

#[op2(fast)]
pub fn op_tracing_info_fast(#[string] message: &str) -> Result<()> {
    emit_op_tracing_event(message, Level::INFO, None)
}

#[op2(fast(op_tracing_info_fast))]
pub fn op_tracing_info(
    #[string] message: &str,
    #[serde] options: Option<TracingOptions>,
) -> Result<()> {
    emit_op_tracing_event(message, Level::INFO, options)
}

#[op2(fast)]
pub fn op_tracing_warn_fast(#[string] message: &str) -> Result<()> {
    emit_op_tracing_event(message, Level::WARN, None)
}

#[op2(fast(op_tracing_warn_fast))]
pub fn op_tracing_warn(
    #[string] message: &str,
    #[serde] options: Option<TracingOptions>,
) -> Result<()> {
    emit_op_tracing_event(message, Level::WARN, options)
}

#[op2(fast)]
pub fn op_tracing_error_fast(#[string] message: &str) -> Result<()> {
    emit_op_tracing_event(message, Level::ERROR, None)
}

#[op2(fast(op_tracing_error_fast))]
pub fn op_tracing_error(
    #[string] message: &str,
    #[serde] options: Option<TracingOptions>,
) -> Result<()> {
    emit_op_tracing_event(message, Level::ERROR, options)
}
