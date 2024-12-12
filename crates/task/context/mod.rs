//! [`TaskRequest`], [`TaskResponse`] and [`TaskError`].

pub mod builders {
    //! [`TaskRequest`] and [`TaskResponse`] builders.
    //!
    //! [`TaskRequest`]: crate::context::TaskRequest
    //! [`TaskResponse`]: crate::context::TaskResponse

    pub use super::request::TaskRequestBuilder;
    pub use super::response::TaskResponseBuilder;
}

pub use crate::context::failure::{TaskError, TaskResult};
pub use crate::context::request::TaskRequest;
pub use crate::context::response::TaskResponse;

mod failure;
mod request;
mod response;
