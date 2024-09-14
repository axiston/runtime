//! [`TaskRequest`] and [`TaskResponse`] types.

pub use crate::context::failure::TaskError;
pub use crate::context::request::TaskRequest;
pub use crate::context::response::TaskResponse;

pub mod builder {
    //! [`TaskRequest`] and [`TaskResponse`] builders.
    //!
    //! [`TaskRequest`]: crate::context::TaskRequest
    //! [`TaskResponse`]: crate::context::TaskResponse

    pub use super::request::TaskRequestBuilder;
    pub use super::response::TaskResponseBuilder;
}

mod failure;
mod request;
mod response;
mod state;
