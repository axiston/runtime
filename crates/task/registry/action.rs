//! Operation [`Request`], [`Response`] and [`Manifest`] types.
//!
//! [`Request`]: ActionRequest
//! [`Response`]: ActionResponse
//! [`Manifest`]: ActionManifest

use serde::{Deserialize, Serialize};

/// TODO.
#[derive(Debug, Serialize, Deserialize)]
pub struct ActionRequest {}

impl ActionRequest {
    /// Returns a new [`ActionRequest`].
    pub fn new() -> Self {
        Self {}
    }
}

/// TODO.
#[derive(Debug, Serialize, Deserialize)]
pub struct ActionResponse {}

impl ActionResponse {
    /// Returns a new [`ActionResponse`].
    pub fn new() -> Self {
        Self {}
    }
}

/// Associated action metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[must_use = "manifests do nothing unless you serialize them"]
pub struct ActionManifest {
    pub name: String,
}

impl ActionManifest {
    /// Returns a new [`ActionManifest`].
    ///
    /// Used for testing.
    #[inline]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }

    // pub fn index() -> Index {}
}

#[cfg(test)]
mod test {
    use tower::{service_fn, ServiceBuilder};

    use crate::context::{TaskError, TaskRequest, TaskResponse};
    use crate::handler::TaskHandlerLayer;
    use crate::registry::action::{ActionRequest, ActionResponse};
    use crate::Result;

    async fn action_handle(
        request: TaskRequest<ActionRequest>,
    ) -> Result<TaskResponse<ActionResponse>, TaskError> {
        Ok(TaskResponse::new(ActionResponse::new()))
    }

    #[tokio::test]
    async fn native_action_handle() -> Result<()> {
        let req = TaskRequest::new(ActionRequest::new());
        let svc = ServiceBuilder::new()
            .layer(TaskHandlerLayer::new())
            .service(service_fn(action_handle));

        Ok(())
    }
}
