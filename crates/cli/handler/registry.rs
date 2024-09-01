use tonic::{Request, Response, Status};

use crate::handler::registry::registry_proto::registry_server::{Registry, RegistryServer};
use crate::handler::registry::registry_proto::{HelloRequest, HelloResponse};
use crate::service::AppState;

pub mod registry_proto {
    tonic::include_proto!("registry");
}

/// TODO.
pub struct RegistryService {
    state: AppState,
}

impl RegistryService {
    /// Returns a new [`RegistryService`].
    #[inline]
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    /// Returns a `GRPC` service.
    #[inline]
    pub fn into_server(self) -> RegistryServer<Self> {
        RegistryServer::new(self)
    }
}

#[tonic::async_trait]
impl Registry for RegistryService {
    async fn hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        todo!()
    }
}
