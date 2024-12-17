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
use std::collections::HashMap;

use derive_more::From;
use jsonschema::ValidationError;

use crate::routing::index::{RouteIndex, ServiceIndex};
use crate::routing::manifest::{RouteManifest, ServiceManifest};

pub mod context;
pub mod handler;
pub mod routing;

/// TODO.
#[derive(Debug)]
pub struct Registry {
    services: HashMap<ServiceIndex, ServiceManifest>,
    routes: HashMap<RouteIndex, RouteManifest>,
}

/// Unrecoverable failure of the [`Router`].
///
/// Includes all error types that may occur.
///
/// [`Router`]: routing::Router
#[derive(Debug, thiserror::Error, From)]
#[must_use = "errors do nothing unless you use them"]
pub enum Error {
    /// Route with the index not found.
    #[error("index not found")]
    Index(RouteIndex),
    /// Task validation failure.
    #[from(ignore)]
    #[error("task validation failure: {0}")]
    Validation(ValidationError<'static>),
    /// Task execution failure.
    #[from(ignore)]
    #[error("task execution failure: {0}")]
    Execution(#[from] context::TaskError),
}

impl<'a> From<ValidationError<'a>> for Error {
    fn from(validation_error: ValidationError<'a>) -> Self {
        let validation_error = ValidationError {
            instance: Cow::Owned(validation_error.instance.into_owned()),
            kind: validation_error.kind,
            instance_path: validation_error.instance_path,
            schema_path: validation_error.schema_path,
        };

        Self::Validation(validation_error)
    }
}

/// Specialized [`Result`] alias for the [`Error`] type.
///
/// [`Result`]: std::result::Result
pub type Result<T, E = Error> = std::result::Result<T, E>;
