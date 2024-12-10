//! Condition [`Request`], [`Response`] and [`Manifest`] types.
//!
//! [`Request`]: TriggerRequest
//! [`Response`]: TriggerResponse
//! [`Manifest`]: TriggerManifest

use std::time::Duration;

use serde::{Deserialize, Serialize};

/// TODO.
#[derive(Debug, Serialize, Deserialize)]
pub struct TriggerRequest {}

impl TriggerRequest {
    /// Returns a new [`TriggerRequest`].
    pub fn new() -> Self {
        Self {}
    }
}

/// TODO.
#[derive(Debug, Serialize, Deserialize)]
pub struct TriggerResponse {
    pub should_trigger: bool,
    pub ignore_retry_ms: Option<Duration>,
}

impl TriggerResponse {
    /// Returns a new [`TriggerResponse`].
    pub fn new(should_trigger: bool) -> Self {
        Self {
            should_trigger,
            ignore_retry_ms: None,
        }
    }

    pub fn with_ignore_retry(self) -> Self {
        todo!()
    }
}

/// Associated trigger metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[must_use = "manifests do nothing unless you serialize them"]
pub struct TriggerManifest {
    pub name: String,
}

impl TriggerManifest {
    /// Returns a new [`TriggerManifest`].
    ///
    /// Used for testing.
    #[inline]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }
}

#[cfg(test)]
mod test {
    use tower::{service_fn, Service, ServiceBuilder};

    use crate::context::{TaskError, TaskRequest, TaskResponse};
    use crate::handler::TaskHandlerLayer;
    use crate::registry::trigger::{TriggerRequest, TriggerResponse};
    use crate::Result;

    async fn trigger_handle(
        request: TaskRequest<TriggerRequest>,
    ) -> Result<TaskResponse<TriggerResponse>, TaskError> {
        let resp = TriggerResponse::new(true);
        Ok(TaskResponse::builder(resp).build())
    }

    #[tokio::test]
    async fn native_trigger_handle() -> Result<()> {
        let req = TaskRequest::new(TriggerRequest::new());
        let mut svc = ServiceBuilder::new()
            .layer(TaskHandlerLayer::new())
            .service(service_fn(trigger_handle));

        let _resp = svc.call(req).await?;
        Ok(())
    }
}
