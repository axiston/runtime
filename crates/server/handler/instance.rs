use axiston_rt_schema::instance::instance_server::{Instance, InstanceServer};
use axiston_rt_schema::instance::{EventRequest, EventResponse};
use futures::stream::BoxStream;
use futures::StreamExt;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};

use crate::service::AppState;

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
    type ConnectStream = BoxStream<'static, Result<EventResponse, Status>>;

    async fn connect(
        &self,
        request: Request<Streaming<EventRequest>>,
    ) -> Result<Response<Self::ConnectStream>, Status> {
        let mut request = request.into_inner();

        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move { while let Some(event) = request.next().await {} });

        let rx = ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(rx)))
    }
}
