use std::sync::Arc;

use jsonschema::{draft202012, Validator};
use tower::load::Load;
use tower::Service;

use crate::context::{TaskRequest, TaskResponse};
use crate::handler::metric::TaskMetrics;
use crate::handler::TaskHandler;
use crate::routing::layers::LayerCompose;
use crate::routing::RouteManifest;
use crate::Result;

/// TODO.
#[must_use = "routes do nothing unless you use them"]
pub struct Route<T, U> {
    pub(crate) route_handler: Arc<RouteHandler<T, U>>,
}

struct RouteHandler<T, U> {
    pub(crate) route_task_handler: TaskHandler<T, U>,
    pub(crate) route_manifest: RouteManifest,
    pub(crate) inputs_schema_validator: Validator,
    pub(crate) outputs_schema_validator: Validator,
    pub(crate) errors_schema_validator: Validator,
}

impl<T, U> Route<T, U> {
    /// Returns a new [`Route`].
    pub fn new(
        route_task_handler: TaskHandler<T, U>,
        layer_compose: Option<LayerCompose>,
        route_manifest: RouteManifest,
    ) -> Result<Self> {
        let route_handler = RouteHandler {
            inputs_schema_validator: draft202012::new(&route_manifest.inputs_schema)?,
            outputs_schema_validator: draft202012::new(&route_manifest.outputs_schema)?,
            errors_schema_validator: draft202012::new(&route_manifest.errors_schema)?,
            route_task_handler,
            route_manifest,
        };

        Ok(Self {
            route_handler: Arc::new(route_handler),
        })
    }

    /// Returns the underlying `tower::`[`Service`].
    #[inline]
    fn task_handler(&self) -> TaskHandler<T, U> {
        self.route_handler.route_task_handler.clone()
    }

    /// Returns the underlying `tower::`[`Service`]'s metrics.
    #[inline]
    pub fn task_handler_metrics(&self) -> TaskMetrics {
        self.route_handler.route_task_handler.load()
    }

    /// Processes the request and returns the response asynchronously.
    pub async fn execute(&self, task_request: TaskRequest<T>) -> Result<TaskResponse<U>> {
        // TODO: Apply layers.
        // let _ = &task_request.layers;

        self.route_handler
            .inputs_schema_validator
            .validate(&task_request.inputs)?;
        let mut task_handler = self.route_handler.route_task_handler.clone();
        let task_response = task_handler.call(task_request).await?;
        self.route_handler
            .outputs_schema_validator
            .validate(&task_response.outputs)?;
        Ok(task_response)
    }
}

impl<T, U> Clone for Route<T, U> {
    fn clone(&self) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod test {}