use tonic::{Request, Response, Status};

use crate::handler::instance::instance_proto::instance_server::{Instance, InstanceServer};
use crate::handler::instance::instance_proto::{HelloRequest, HelloResponse};
use crate::service::AppState;

pub mod instance_proto {
    tonic::include_proto!("instance");
}

/// TODO.
pub struct InstanceService {
    state: AppState,
}

impl InstanceService {
    /// Returns a new [`InstanceService`].
    #[inline]
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    /// Returns a `GRPC` service.
    #[inline]
    pub fn into_server(self) -> InstanceServer<Self> {
        InstanceServer::new(self)
    }
}

#[tonic::async_trait]
impl Instance for InstanceService {
    async fn hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        todo!()
    }
}
