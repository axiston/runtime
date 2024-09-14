//! TODO.
//!

use crate::datatype::action::{ActionManifest, ActionRequest, ActionResponse};
use crate::datatype::trigger::{TriggerManifest, TriggerRequest, TriggerResponse};
use tower_path::service::WithData;
use tower_task::handler::TaskHandler;

/// TODO.
pub type TriggerHandler = WithData<TaskHandler<TriggerRequest, TriggerResponse>, TriggerManifest>;

/// TODO.
pub type ActionHandler = WithData<TaskHandler<ActionRequest, ActionResponse>, ActionManifest>;

#[cfg(test)]
mod test {
    use crate::Result;
    use tower::{service_fn, ServiceBuilder};
    use tower_path::service::{WithData, WithDataLayer};

    async fn handle(request: u32) -> Result<u32> {
        Ok(request)
    }

    #[test]
    fn service() -> tower_task::Result<()> {
        let inner = service_fn(handle);
        let _ = WithData::new(inner, 42u32);
        Ok(())
    }

    #[test]
    fn layer() -> tower_task::Result<()> {
        let _ = ServiceBuilder::new()
            .layer(WithDataLayer::new(42u32))
            .service(service_fn(handle));
        Ok(())
    }
}
