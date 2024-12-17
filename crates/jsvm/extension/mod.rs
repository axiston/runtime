//! Runtime `deno_core::`[`extension`]s.
//!

pub use crate::extension::route::axis_routing;
pub use crate::extension::trace::axis_tracing;

mod route;
mod trace;
