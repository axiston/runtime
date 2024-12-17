use axiston_rt_schema::registry::registry_server::{Registry, RegistryServer};
use axiston_rt_schema::registry::{
    FindServicesRequest, FindServicesResponse, RegistryContentResponse,
};
// use axiston_rt_schema::registry::{CheckRequest, CheckResponse, RegistryRequest, RegistryResponse};
use tonic::{Request, Response, Status};

use crate::service::AppState;

/// Implements [`Registry`] service for the [`RegistryServer`].
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
    async fn get_registry_content(
        &self,
        request: Request<()>,
    ) -> Result<Response<RegistryContentResponse>, Status> {
        todo!()
    }

    async fn find_services(
        &self,
        request: Request<FindServicesRequest>,
    ) -> Result<Response<FindServicesResponse>, Status> {
        todo!()
    }
}
