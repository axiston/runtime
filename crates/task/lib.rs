#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("./README.md")]

//! ```rust
//! use axiston_rt_task::routing::Router;
//! use axiston_rt_task::Result;
//!
//! fn main() -> Result<()> {
//!     let router: Router = Router::default();
//!     Ok(())
//! }
//! ```

use std::borrow::Cow;

use jsonschema::ValidationError;

pub mod context;
pub mod handler;
pub mod routing;

/// Unrecoverable failure of the [`Router`].
///
/// Includes all error types that may occur.
///
/// [`Router`]: routing::Router
#[derive(Debug, thiserror::Error)]
#[must_use = "errors do nothing unless you use them"]
pub enum Error {
    /// Task validation failure.
    #[error("task validation failure: {0}")]
    Validate(ValidationError<'static>),
    /// Task execution failure.
    #[error("task execution failure: {0}")]
    Task(#[from] context::TaskError),
}

impl<'a> From<ValidationError<'a>> for Error {
    fn from(validation_error: ValidationError<'a>) -> Self {
        let validation_error = ValidationError {
            instance: Cow::Owned(validation_error.instance.into_owned()),
            kind: validation_error.kind,
            instance_path: validation_error.instance_path,
            schema_path: validation_error.schema_path,
        };

        Self::Validate(validation_error)
    }
}

/// Specialized [`Result`] alias for the [`Error`] type.
///
/// [`Result`]: std::result::Result
pub type Result<T, E = Error> = std::result::Result<T, E>;

// TODO: Is there any real reason to make a different between action and trigger?
// Make trigger dynamically determined if the action returns a single boolean.

// TODO: Manifests are defined in proto files with serde derive.
