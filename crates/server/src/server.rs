use crate::api::api::multi_sig_service_server::MultiSigServiceServer;
use crate::service::GrpcService;
use anyhow::Result;
use config::Config;
use xactor::*;

pub(crate) struct Server {
    config: Config,
}

#[async_trait::async_trait]
impl Actor for Server {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        info!("Server system service starting...");
        Ok(())
    }

    async fn stopped(&mut self, _ctx: &mut Context<Self>) {
        info!("Server system service stopped.");
    }
}

impl Service for Server {}
impl Default for Server {
    fn default() -> Self {
        Server {
            config: Config::default(),
        }
    }
}

#[message(result = "Result<(Config)>")]
pub(crate) struct GetConfig;

/// Get the current pos compute config
#[async_trait::async_trait]
impl Handler<GetConfig> for Server {
    async fn handle(&mut self, _ctx: &mut Context<Self>, _msg: GetConfig) -> Result<Config> {
        Ok(self.config.clone())
    }
}

#[message(result = "Result<()>")]
pub(crate) struct SetConfig(pub(crate) Config);

/// Set the pos compute config
#[async_trait::async_trait]
impl Handler<SetConfig> for Server {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: SetConfig) -> Result<()> {
        self.config = msg.0;
        Ok(())
    }
}

/////////////////////////////////////////////

#[message(result = "Result<()>")]
pub(crate) struct StartGrpcService {
    pub(crate) port: u32,
    pub(crate) host: String,
}
#[async_trait::async_trait]
impl Handler<StartGrpcService> for Server {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: StartGrpcService) -> Result<()> {
        let addr = format!("{}:{}", msg.host, msg.port).parse().unwrap();
        info!("starting grpc service on: {}...", addr);

        // todo: add a grpc health service
        tokio::task::spawn(async move {
            let res = tonic::transport::Server::builder()
                .add_service(MultiSigServiceServer::new(GrpcService::default()))
                .serve(addr)
                .await;
            if res.is_err() {
                panic!("grpc server stopped due to error: {:?}", res.err().unwrap());
            } else {
                info!("grpc server stopped");
            }
        });

        Ok(())
    }
}
