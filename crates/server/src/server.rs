use crate::api::api::multi_sig_service_server::MultiSigServiceServer;
use crate::service::GrpcService;
use anyhow::{anyhow, bail, Result};
use api::api::StoreMessageRequest;
use config::Config;
use rocksdb::DB;
use std::collections::HashSet;
use xactor::*;

const MAX_ADDRESS_SIZE_BYTES: usize = 128;
const MAX_TX_DATA_SIZE_BYTES: usize = 2048;
const ALL_ADDRESSES_KEY: &str = "all_addresses";
const DB_FILE_PATH: &str = "./data.bin";
pub(crate) struct Server {
    config: Config,
    db: Option<DB>,
}

#[async_trait::async_trait]
impl Actor for Server {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        info!("Server system service starting...");
        self.db = Some(DB::open_default(DB_FILE_PATH)?);
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
            db: None,
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

//////////////////

#[message(result = "Result<()>")]
pub(crate) struct SetConfig(pub(crate) Config);

/// Set server config
#[async_trait::async_trait]
impl Handler<SetConfig> for Server {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: SetConfig) -> Result<()> {
        self.config = msg.0;
        Ok(())
    }
}

/////////////////////////////////////////////

#[message(result = "Result<()>")]
pub(crate) struct StoreMessage(pub(crate) StoreMessageRequest);

/// Set server config
#[async_trait::async_trait]
impl Handler<StoreMessage> for Server {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: StoreMessage) -> Result<()> {
        // validate all input
        let user_msg = msg
            .0
            .user_message
            .ok_or_else(|| anyhow!("invalid input: missing user message"))?;

        let address = user_msg.address;
        if address.is_empty() || address.len() > MAX_ADDRESS_SIZE_BYTES {
            bail!("invalid input: address size failed validation")
        }

        // todo: ensure address != ALL_ADDRESSES_KEY here

        // todo: verify t is not too much in the future compared to server wall time
        let _t = user_msg.created;

        let tx_data = user_msg.transaction_data;
        if tx_data.is_empty() || tx_data.len() > MAX_TX_DATA_SIZE_BYTES {
            bail!("invalid input: transaction data failed validation")
        }

        // todo: verify that tx_data is signed by the private key matching one of the multi-sig addresses for an account
        // or a smart contract by using the Spacemesh public API to get these addresses from a network.

        // input data is valid - store it
        // we store the tx_data in a vector indexed by address
        if let Some(db) = self.db.as_ref() {
            match db.get(address.clone()) {
                Ok(Some(data)) => {
                    let mut messages: Vec<Vec<u8>> = bincode::deserialize(data.as_ref())?;
                    messages.push(tx_data);
                    let encoded_messages: Vec<u8> = bincode::serialize(&messages)?;
                    db.put(address.clone(), encoded_messages)?;
                }
                Ok(None) => {
                    let mut messages: Vec<Vec<u8>> = vec![Vec::new()];
                    messages.push(tx_data);
                    let encoded_messages: Vec<u8> = bincode::serialize(&messages)?;
                    db.put(address.clone(), encoded_messages)?;
                }
                Err(e) => {
                    error!("failed db get: {}", e);
                    bail!("internal data error")
                }
            }
            // Add address (e.g. vault's address) to global addresses hashset. Used to prune old messages from the db.
            match db.get(ALL_ADDRESSES_KEY) {
                Ok(Some(data)) => {
                    let mut addresses: HashSet<Vec<u8>> = bincode::deserialize(data.as_ref())?;
                    addresses.insert(address);
                    let encoded_addresses: Vec<u8> = bincode::serialize(&addresses)?;
                    db.put(ALL_ADDRESSES_KEY, encoded_addresses)?;
                }
                Ok(None) => {
                    let mut addresses: HashSet<Vec<u8>> = HashSet::default();
                    addresses.insert(address);
                    let encoded_addresses: Vec<u8> = bincode::serialize(&addresses)?;
                    db.put(ALL_ADDRESSES_KEY, encoded_addresses)?;
                }
                Err(e) => {
                    error!("failed db get: {}", e);
                    bail!("internal data error")
                }
            }
        } else {
            error!("internal state error - db is none");
            bail!("internal data error")
        }

        Ok(())
    }
}

////////////////////////////////

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

////////////////////////////////
