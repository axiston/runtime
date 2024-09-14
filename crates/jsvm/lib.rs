#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("./README.md")]

//! ### Examples
//!
//! ```rust
//! fn main() {}
//! ```

mod extension;

/// Unrecoverable failure of the [`Jsvm`].
///
/// Includes all error types that may occur.
///
/// [`Jsvm`]: runtime::Jsvm
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
