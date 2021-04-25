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
    /*
    async fn get_providers(
        &self,
        _request: Request<GetProvidersRequest>,
    ) -> Result<Response<GetProvidersResponse>, Status> {
        let server = PosServer::from_registry()
            .await
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?;

        let providers: Vec<Provider> = server
            .call(GetAllProviders {})
            .await
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?;

        Ok(Response::new(GetProvidersResponse { providers }))
    }

    async fn set_config(
        &self,
        request: Request<SetConfigRequest>,
    ) -> Result<Response<SetConfigResponse>, Status> {
        let config = request
            .into_inner()
            .config
            .ok_or_else(|| Status::invalid_argument("missing config"))?;

        let server = PosServer::from_registry()
            .await
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?;

        server
            .call(SetConfig(config))
            .await
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?;

        Ok(Response::new(SetConfigResponse {}))
    }

    async fn get_config(
        &self,
        _request: Request<GetConfigRequest>,
    ) -> Result<Response<GetConfigResponse>, Status> {
        let server = PosServer::from_registry()
            .await
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?;

        let config = server
            .call(GetConfig {})
            .await
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?;

        Ok(Response::new(GetConfigResponse {
            config: Some(config),
        }))
    }

    async fn add_job(
        &self,
        request: Request<AddJobRequest>,
    ) -> Result<Response<AddJobResponse>, Status> {
        let add_job_request = request.into_inner();

        let server = PosServer::from_registry()
            .await
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?;

        let job: Job = server
            .call(AddJob(add_job_request))
            .await
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?;

        Ok(Response::new(AddJobResponse { job: Some(job) }))
    }

    async fn get_job_status(
        &self,
        request: Request<GetJobStatusRequest>,
    ) -> Result<Response<GetJobStatusResponse>, Status> {
        let id = request.into_inner().id;

        let server = PosServer::from_registry()
            .await
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?;

        let res: Option<Job> = server
            .call(GetJob(id))
            .await
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?;

        Ok(Response::new(GetJobStatusResponse { job: res }))
    }

    async fn get_all_jobs_statuses(
        &self,
        _request: Request<GetAllJobsStatusRequest>,
    ) -> Result<Response<GetAllJobsStatusResponse>, Status> {
        let server = PosServer::from_registry()
            .await
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?;

        let jobs: Vec<Job> = server
            .call(GetAllJobs {})
            .await
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?;

        Ok(Response::new(GetAllJobsStatusResponse { jobs }))
    }

    async fn abort_job(
        &self,
        request: Request<AbortJobRequest>,
    ) -> Result<Response<AbortJobResponse>, Status> {
        let server = PosServer::from_registry()
            .await
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?;

        server
            .call(AbortJob(request.into_inner()))
            .await
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?;

        Ok(Response::new(AbortJobResponse {}))
    }

    type SubscribeJobStatusStreamStream = ReceiverStream<Result<JobStatusStreamResponse, Status>>;

    async fn subscribe_job_status_stream(
        &self,
        _request: Request<JobStatusStreamRequest>,
    ) -> Result<Response<Self::SubscribeJobStatusStreamStream>, Status> {
        let server = PosServer::from_registry()
            .await
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?;

        let resp: Self::SubscribeJobStatusStreamStream = server
            .call(SubscribeToJobStatuses {})
            .await
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?;

        Ok(Response::new(resp))
    }*/
}
