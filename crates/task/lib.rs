#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("./README.md")]

//! ```rust
//! ```

pub mod datatype;
pub mod registry;

/// Unrecoverable failure of the [`Router`].
///
/// Includes all error types that may occur.
///
/// [`Router`]: registry::Router
#[derive(Debug, thiserror::Error)]
#[must_use = "errors do nothing unless you use them"]
pub enum Error {
    // #[error("called task failure: {0}")]
    // Task(#[from] context::TaskError),
}

/// Specialized [`Result`] alias for the [`Error`] type.
///
/// [`Result`]: std::result::Result
pub type Result<T, E = Error> = std::result::Result<T, E>;
