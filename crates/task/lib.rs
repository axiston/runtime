#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("./README.md")]

//! ```rust
//! use axiston_rt_task::routing::Router;
//! use axiston_rt_task::Result;
//!
//! fn main() -> Result<()> {
//!     let router = Router::default();
//!     Ok(())
//! }
//! ```

pub mod context;
pub mod handler;
pub mod registry;
pub mod routing;

/// Unrecoverable failure of the [`Registry`].
///
/// Includes all error types that may occur.
///
/// [`Registry`]: registry::Registry
#[derive(Debug, thiserror::Error)]
#[must_use = "errors do nothing unless you use them"]
pub enum Error {
    #[error("called task failure: {0}")]
    Task(#[from] context::TaskError),
}

/// Specialized [`Result`] alias for the [`Error`] type.
///
/// [`Result`]: std::result::Result
pub type Result<T, E = Error> = std::result::Result<T, E>;
