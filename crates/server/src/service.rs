use crate::api::api::multi_sig_service_server::MultiSigService;

use crate::server::{Server, StoreMessage};
use anyhow::Result;
use api::api::{
    GetMessagesRequest, GetMessagesResponse, StoreMessageRequest, StoreMessageResponse,
};
use tonic::{Request, Response, Status};
use xactor::Service;
// use xactor::*;

#[derive(Debug)]
pub(crate) struct GrpcService {}

impl Default for GrpcService {
    fn default() -> Self {
        debug!("PosService grpc service started");
        GrpcService {}
    }
}

impl GrpcService {}

#[tonic::async_trait]
impl MultiSigService for GrpcService {
    async fn store_message(
        &self,
        request: Request<StoreMessageRequest>,
    ) -> Result<Response<StoreMessageResponse>, Status> {
        let server = Server::from_registry()
            .await
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?;

        let _ = server
            .call(StoreMessage(request.into_inner()))
            .await
            .map_err(|e| Status::internal(format!("internal call error: {}", e)))?
            .map_err(|e| Status::internal(format!("error: {}", e)))?;

        Ok(Response::new(StoreMessageResponse {}))
    }

    async fn get_messages(
        &self,
        _request: Request<GetMessagesRequest>,
    ) -> Result<Response<GetMessagesResponse>, Status> {
        todo!()
    }
}
