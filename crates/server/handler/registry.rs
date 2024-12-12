use axiston_rt_schema::registry::registry_server::{Registry, RegistryServer};
use axiston_rt_schema::registry::{CheckRequest, CheckResponse, RegistryRequest, RegistryResponse};
use tonic::{Request, Response, Status};

use crate::service::AppState;

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
    async fn registry(
        &self,
        request: Request<RegistryRequest>,
    ) -> Result<Response<RegistryResponse>, Status> {
        todo!()
    }

    async fn check(
        &self,
        request: Request<CheckRequest>,
    ) -> Result<Response<CheckResponse>, Status> {
        todo!()
    }
}
