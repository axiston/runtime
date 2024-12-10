//! TODO.
//!

use tower::ServiceBuilder;

pub use crate::middleware::observability::initialize_tracing;
mod observability;
mod utility;

/// Extension trait for `tower::`[`ServiceBuilder`] for layering middleware.
pub trait ServiceBuilderExt<L> {}

impl<L> ServiceBuilderExt<L> for ServiceBuilder<L> {}
