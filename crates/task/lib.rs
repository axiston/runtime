#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("./README.md")]

//! ```rust
//! use tower::{service_fn, Service};
//! use runtime_task::context::{TaskError, TaskRequest, TaskResponse};
//! use runtime_task::handler::TaskHandler;
//! use runtime_task::routing::{Index, Layers, Router};
//!
//! async fn handle(request: TaskRequest<u32>) -> Result<TaskResponse<u32>, TaskError> {
//!     Ok(TaskResponse::new(request.into_inner()))
//! }
//!
//! let layers = Layers::default();
//! let mut router = Router::new(layers);
//!
//! let index = Index::new("task01");
//! let handler = TaskHandler::new((), service_fn(handle));
//! router.route(index.clone(), handler);
//!
//! let fut = async {
//!     let request = TaskRequest::new(index, 0u32);
//!     let response = router.call(request).await;
//! };
//!
//! ```

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
    #[error("called task failure: {0}")]
    Task(#[from] context::TaskError), // Task not found
                                      // Mismatch arguments
}

/// Specialized [`Result`] alias for the [`Error`] type.
///
/// [`Result`]: std::result::Result
pub type Result<T, E = Error> = std::result::Result<T, E>;
