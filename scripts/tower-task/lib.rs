#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("./README.md")]

//! ### Examples
//!
//! ```rust
//! use tower_task::Result;
//!
//! fn main() -> Result<()> {
//!     Ok(())
//! }
//! ```

pub mod context;
pub mod handler;
pub mod compose;

/// Specialized [`Result`] alias for the [`TaskError`] type.
///
/// [`TaskError`]: crate::context::TaskError
/// [`Result`]: std::result::Result
pub type Result<T, E = context::TaskError> = std::result::Result<T, E>;

// TODO: Move tower-task into its own repository.
// .github/dependabot.yaml,.github/workflows, rustfmt.toml
