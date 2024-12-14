use std::num::{NonZero, NonZeroU32};

use axiston_rt_schema::instance::event_request::Payload as RequestPayload;
use axiston_rt_schema::instance::event_response::Payload as ResponsePayload;
use axiston_rt_schema::instance::instance_server::{Instance, InstanceServer};
use axiston_rt_schema::instance::{
    EventRequest, EventResponse, GetStatusRequest, GetStatusResponse,
};
use axiston_rt_schema::response::OpenResponse;
use futures::stream::BoxStream;
use futures::{Stream, StreamExt};
use prost_types::Timestamp;
use time::OffsetDateTime;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};

use crate::handler::ErrorKind;
use crate::service::AppState;

/// Implements [`Instance`] service for the [`InstanceService`].
#[derive(Clone)]
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

    /// TODO.
    async fn process_event_payload(
        &self,
        payload: RequestPayload,
    ) -> Result<ResponsePayload, Status> {
        match payload {
            RequestPayload::OpenRequest(_) => {
                // let _guard = task_counter.guard_running_tasks();
                // task_counter.
            }
            RequestPayload::PolicyRequest(_) => {}
            RequestPayload::ExecuteRequest(_) => {}
            RequestPayload::CloseRequest(_) => {}
        }

        todo!()
    }
}

#[tonic::async_trait]
impl Instance for InstanceService {
    async fn get_status(
        &self,
        request: Request<GetStatusRequest>,
    ) -> Result<Response<GetStatusResponse>, Status> {
        let (metadata, extension, request) = request.into_parts();

        let sliding_window = request.sliding_window.and_then(NonZeroU32::new);
        // let snapshot = self.state.task_counter.get_snapshot(sliding_window);

        let message = GetStatusResponse {
            tasks_waiting: 0,
            tasks_running: 0,
            recent_waiting_time: None,
            recent_running_time: None,
            average_waiting_time: None,
            average_running_time: None,
        };

        // Ok(Response::new(message))

        todo!()
    }

    type ConnectWorkerStream = BoxStream<'static, Result<EventResponse, Status>>;

    async fn connect_worker(
        &self,
        request: Request<Streaming<EventRequest>>,
    ) -> Result<Response<Self::ConnectWorkerStream>, Status> {
        let mut request = request.into_inner();
        let (tx, rx) = mpsc::channel(128);

        // TODO: Create a new queue.
        // let task_queue = self.state.task_queue.clone();
        // let task_counter = self.state.task_counter.clone();

        let this = self.clone();
        let fut = async move {
            while let Some(request) = request.next().await {
                let recv_data_time = OffsetDateTime::now_utc();
                let recv_timestamp = Timestamp {
                    seconds: recv_data_time.unix_timestamp(),
                    nanos: recv_data_time.nanosecond() as i32,
                };

                let event_request = match request {
                    Ok(event_request) => event_request,
                    Err(event_status) => todo!(),
                };

                let Some(request_payload) = event_request.payload else {
                    tx.send(Err(ErrorKind::Unknown.into_status())).await;
                    continue;
                };

                let response_payload = match request_payload {
                    RequestPayload::OpenRequest(_) => {
                        ResponsePayload::OpenResponse(OpenResponse {})
                    }
                    RequestPayload::PolicyRequest(_) => todo!(),
                    RequestPayload::ExecuteRequest(_) => todo!(),
                    RequestPayload::CloseRequest(_) => todo!(),
                };

                let send_data_time = OffsetDateTime::now_utc();
                let send_timestamp = Timestamp {
                    seconds: send_data_time.unix_timestamp(),
                    nanos: send_data_time.nanosecond() as i32,
                };

                let event_response = EventResponse {
                    request_id: event_request.request_id,
                    group_id: event_request.group_id,
                    response_id: 0,
                    recv: Some(recv_timestamp),
                    send: Some(send_timestamp),
                    payload: Some(response_payload),
                };

                tx.send(Ok(event_response)).await;
            }

            // Client closed connection.
        };

        let _handle = tokio::spawn(fut);
        let rx = ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(rx)))
    }
}
