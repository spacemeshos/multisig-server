use crate::api::api::multi_sig_service_server::MultiSigService;

use anyhow::Result;
use api::api::{
    GetMessagesRequest, GetMessagesResponse, StoreMessageRequest, StoreMessageResponse,
};
use tonic::{Request, Response, Status};
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
        _request: Request<StoreMessageRequest>,
    ) -> Result<Response<StoreMessageResponse>, Status> {
        todo!()
    }

    async fn get_messages(
        &self,
        _request: Request<GetMessagesRequest>,
    ) -> Result<Response<GetMessagesResponse>, Status> {
        todo!()
    }
}
