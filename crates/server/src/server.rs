use crate::MSG_RETENTION_DUR_CONFIG_KEY_NAME;
use anyhow::{anyhow, bail, Result};
use api::api::{GetMessagesRequest, StoreMessageRequest, UserMessage};
use chrono::prelude::*;
use config::Config;
use prost::Message;
use rocksdb::{Options, DB};
use std::collections::HashSet;
use xactor::*;

const MAX_ADDRESS_SIZE_BYTES: usize = 128;
const MAX_TX_DATA_SIZE_BYTES: usize = 2048;
const ALL_ADDRESSES_KEY: &[u8] = b"all_addresses";
const DB_FILE_PATH: &str = "./data_store";
// new messages with creation time bigger than window relative to server time will be rejected
const ACCEPTED_MESSAGES_TIME_WINDOW_SECS: i64 = 60 * 60 * 24;

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
        info!("Server system service stopped");
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

//////////////////

#[message(result = "Result<()>")]
pub(crate) struct DeleteDb;

/// Get the current pos compute config
#[async_trait::async_trait]
impl Handler<DeleteDb> for Server {
    async fn handle(&mut self, _ctx: &mut Context<Self>, _msg: DeleteDb) -> Result<()> {
        let _ = DB::destroy(&Options::default(), DB_FILE_PATH);
        let _ = std::fs::remove_dir_all(DB_FILE_PATH);
        Ok(())
    }
}

//////////////////

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

//////////////////

#[message(result = "Result<Vec<UserMessage>>")]
pub(crate) struct GetMessages(pub(crate) GetMessagesRequest);

/// Get all messages for an address
#[async_trait::async_trait]
impl Handler<GetMessages> for Server {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetMessages,
    ) -> Result<Vec<UserMessage>> {
        let address = msg.0.address;

        if let Some(db) = self.db.as_ref() {
            match db.get(address) {
                Ok(Some(data)) => {
                    let messages: Vec<Vec<u8>> = bincode::deserialize(data.as_ref())?;
                    let mut res: Vec<UserMessage> = vec![];
                    for m in messages {
                        res.push(UserMessage::decode(m.as_ref())?);
                    }
                    Ok(res)
                }
                Ok(None) => Ok(vec![]),
                Err(e) => {
                    error!("failed db get: {}", e);
                    bail!("internal data error")
                }
            }
        } else {
            error!("internal state error - db is none");
            bail!("internal data error")
        }
    }
}

///////////////////////

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

        let address = &user_msg.address;
        if address.is_empty() || address.len() > MAX_ADDRESS_SIZE_BYTES {
            bail!("invalid input: address size failed validation")
        }

        if *address == *ALL_ADDRESSES_KEY {
            bail!("invalid input: address failed validation")
        }

        // verify that message creation time is not outside of the server acceptable time window
        let now = Utc::now().timestamp() as i64;
        let t = user_msg.created as i64;
        if i64::abs(now - t) > ACCEPTED_MESSAGES_TIME_WINDOW_SECS {
            bail!("invalid input: message creation time outside of acceptable server time window")
        }

        let tx_data = &user_msg.transaction_data;
        if tx_data.is_empty() || tx_data.len() > MAX_TX_DATA_SIZE_BYTES {
            bail!("invalid input: transaction data failed validation")
        }

        // todo: verify that tx_data is signed by the private key matching one of the multi-sig addresses for an account
        // or a smart contract by using the Spacemesh public API to get these addresses from a network.

        let mut user_msg_bin: Vec<u8> = Vec::with_capacity(user_msg.encoded_len());
        user_msg.encode(&mut user_msg_bin)?;

        // input data is valid - store it
        // we store UserMessage in a vector indexed by address
        if let Some(db) = self.db.as_ref() {
            match db.get(address.clone()) {
                Ok(Some(data)) => {
                    let mut messages: Vec<Vec<u8>> = bincode::deserialize(data.as_ref())?;
                    messages.push(user_msg_bin);
                    let encoded_messages: Vec<u8> = bincode::serialize(&messages)?;
                    db.put(address.clone(), encoded_messages)?;
                }
                Ok(None) => {
                    let messages: Vec<Vec<u8>> = vec![user_msg_bin];
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
                    addresses.insert(address.clone());
                    let encoded_addresses: Vec<u8> = bincode::serialize(&addresses)?;
                    db.put(ALL_ADDRESSES_KEY, encoded_addresses)?;
                }
                Ok(None) => {
                    let mut addresses: HashSet<Vec<u8>> = HashSet::default();
                    addresses.insert(address.clone());
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

//////////////////

#[message(result = "Result<()>")]
pub(crate) struct DeleteOldMessages;

/// Delete old messages from the service
#[async_trait::async_trait]
impl Handler<DeleteOldMessages> for Server {
    async fn handle(&mut self, _ctx: &mut Context<Self>, _msg: DeleteOldMessages) -> Result<()> {
        info!("delete old messages task...");

        let now = Utc::now().timestamp() as u64;
        let retention_duration = self
            .config
            .get_int(MSG_RETENTION_DUR_CONFIG_KEY_NAME)
            .unwrap() as u64;

        if let Some(db) = self.db.as_ref() {
            match db.get(ALL_ADDRESSES_KEY) {
                Ok(Some(data)) => {
                    let mut addresses: HashSet<Vec<u8>> = bincode::deserialize(data.as_ref())?;

                    // addresses that should be removed from the db as they have no messages after messages deletion
                    let mut remove_addresses: HashSet<Vec<u8>> = HashSet::new();
                    for address in addresses.iter() {
                        match db.get(address.clone()) {
                            Ok(Some(data)) => {
                                let messages: Vec<Vec<u8>> = bincode::deserialize(data.as_ref())?;
                                // only keep messages that are not too old
                                let new_messages = messages
                                    .into_iter()
                                    .filter(|m| {
                                        // we can unwrap because we ensured that only UserMessages were inserted previously
                                        let user_msg: UserMessage =
                                            UserMessage::decode(m.as_slice()).unwrap();

                                        user_msg.created >= now - retention_duration
                                    })
                                    .collect::<Vec<_>>();

                                if new_messages.is_empty() {
                                    // no messages for this address - delete the address from the db
                                    db.delete(address.clone())?;
                                    remove_addresses.insert(address.clone());
                                } else {
                                    // store messages for this address excluding the old deleted messages
                                    let encoded_messages: Vec<u8> =
                                        bincode::serialize(&new_messages)?;
                                    db.put(address.clone(), encoded_messages)?;
                                }
                            }
                            Ok(None) => {
                                warn!("no messages found for address in index {:?}", address)
                            }
                            Err(e) => {
                                error!("failed db get: {}", e);
                                bail!("internal data error")
                            }
                        }
                    }

                    // update the addresses global index based on removed addresses
                    if !remove_addresses.is_empty() {
                        for a in remove_addresses.iter() {
                            addresses.remove(a);
                        }
                        let encoded_addresses: Vec<u8> = bincode::serialize(&addresses)?;
                        db.put(ALL_ADDRESSES_KEY, encoded_addresses)?;
                    }
                }
                Ok(None) => {
                    info!("No messages stored");
                    return Ok(());
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

#[cfg(test)]
mod tests {

    use super::*;
    use crate::get_default_config;
    use api::api::TransactionType;
    use log::LevelFilter;
    use serial_test::*;

    fn setup_test() {
        // enable logging
        let _ = env_logger::builder()
            .is_test(false)
            .filter_level(LevelFilter::Info)
            .try_init();

        // delete the db
        let _ = std::fs::remove_dir_all(DB_FILE_PATH);
    }

    #[tokio::test]
    #[serial]
    async fn test_server_service() {
        setup_test();

        let server = Server::from_registry().await.unwrap();
        let config = get_default_config();
        let _ = server.call(SetConfig(config)).await.unwrap().unwrap();

        let address1: Vec<u8> = (0..32).map(|_| rand::random::<u8>()).collect();
        let address2: Vec<u8> = (0..48).map(|_| rand::random::<u8>()).collect();
        let _address3: Vec<u8> = (0..24).map(|_| rand::random::<u8>()).collect();

        let tx1: Vec<u8> = (0..1024).map(|_| rand::random::<u8>()).collect();
        let tx2: Vec<u8> = (0..1000).map(|_| rand::random::<u8>()).collect();
        let tx3: Vec<u8> = (0..150).map(|_| rand::random::<u8>()).collect();

        let t1 = Utc::now().timestamp() as u64;
        let net_id = 1;

        let _ = server
            .call(StoreMessage(StoreMessageRequest {
                user_message: Some(UserMessage {
                    net_id,
                    created: t1,
                    address: address1.clone(),
                    transaction_type: TransactionType::VaultWithdraw as i32,
                    transaction_data: tx1.clone(),
                }),
            }))
            .await
            .unwrap()
            .unwrap();

        let messages: Vec<UserMessage> = server
            .call(GetMessages(GetMessagesRequest {
                address: address1.clone(),
            }))
            .await
            .unwrap()
            .unwrap();

        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].address, address1);
        assert_eq!(messages[0].created, t1);
        assert_eq!(messages[0].transaction_data, tx1);
        assert_eq!(messages[0].net_id, net_id);

        let t2 = Utc::now().timestamp() as u64;

        let _ = server
            .call(StoreMessage(StoreMessageRequest {
                user_message: Some(UserMessage {
                    net_id,
                    created: t2,
                    address: address1.clone(),
                    transaction_type: TransactionType::VaultWithdraw as i32,
                    transaction_data: tx2.clone(),
                }),
            }))
            .await
            .unwrap()
            .unwrap();

        let messages: Vec<UserMessage> = server
            .call(GetMessages(GetMessagesRequest {
                address: address1.clone(),
            }))
            .await
            .unwrap()
            .unwrap();

        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].address, address1);
        assert_eq!(messages[0].created, t1);
        assert_eq!(messages[0].transaction_data, tx1);
        assert_eq!(messages[0].net_id, net_id);

        assert_eq!(messages[1].address, address1);
        assert_eq!(messages[1].created, t2);
        assert_eq!(messages[1].transaction_data, tx2);
        assert_eq!(messages[1].net_id, net_id);

        let t3 = Utc::now().timestamp() as u64;

        let _ = server
            .call(StoreMessage(StoreMessageRequest {
                user_message: Some(UserMessage {
                    net_id,
                    created: t3,
                    address: address2.clone(),
                    transaction_type: TransactionType::VaultWithdraw as i32,
                    transaction_data: tx3.clone(),
                }),
            }))
            .await
            .unwrap()
            .unwrap();

        let messages: Vec<UserMessage> = server
            .call(GetMessages(GetMessagesRequest {
                address: address2.clone(),
            }))
            .await
            .unwrap()
            .unwrap();

        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].address, address2);
        assert_eq!(messages[0].created, t2);
        assert_eq!(messages[0].transaction_data, tx3);
        assert_eq!(messages[0].net_id, net_id);

        // cleanup
        let _ = server.call(DeleteDb {}).await.unwrap().unwrap();
    }

    #[tokio::test]
    #[serial]
    async fn verify_messages_pruning() {
        setup_test();
        let server = Server::from_registry().await.unwrap();

        // set messages retention policy to 10 seconds
        let mut config = get_default_config();
        let c = config
            .set_default(MSG_RETENTION_DUR_CONFIG_KEY_NAME, 10)
            .unwrap()
            .clone();
        let _ = server.call(SetConfig(c)).await.unwrap().unwrap();
        let address1: Vec<u8> = (0..32).map(|_| rand::random::<u8>()).collect();
        let tx1: Vec<u8> = (0..1024).map(|_| rand::random::<u8>()).collect();
        let t1 = Utc::now().timestamp() as u64;
        let net_id = 1;

        let _ = server
            .call(StoreMessage(StoreMessageRequest {
                user_message: Some(UserMessage {
                    net_id,
                    created: t1,
                    address: address1.clone(),
                    transaction_type: TransactionType::VaultWithdraw as i32,
                    transaction_data: tx1.clone(),
                }),
            }))
            .await
            .unwrap()
            .unwrap();

        let _ = server.call(DeleteOldMessages {}).await.unwrap().unwrap();
        let messages: Vec<UserMessage> = server
            .call(GetMessages(GetMessagesRequest {
                address: address1.clone(),
            }))
            .await
            .unwrap()
            .unwrap();

        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].address, address1);
        assert_eq!(messages[0].created, t1);
        assert_eq!(messages[0].transaction_data, tx1);
        assert_eq!(messages[0].net_id, net_id);

        // sleep for 11 seconds
        tokio::time::sleep(tokio::time::Duration::from_secs(11)).await;

        let _ = server.call(DeleteOldMessages {}).await.unwrap().unwrap();
        let messages: Vec<UserMessage> = server
            .call(GetMessages(GetMessagesRequest {
                address: address1.clone(),
            }))
            .await
            .unwrap()
            .unwrap();

        assert_eq!(messages.len(), 0);

        // cleanup
        let _ = server.call(DeleteDb {}).await.unwrap().unwrap();
    }

    #[tokio::test]
    #[serial]
    async fn reject_large_messages() {
        setup_test();

        let server = Server::from_registry().await.unwrap();
        let _ = server
            .call(SetConfig(get_default_config()))
            .await
            .unwrap()
            .unwrap();
        let address1: Vec<u8> = (0..32).map(|_| rand::random::<u8>()).collect();
        let tx1: Vec<u8> = (0..MAX_TX_DATA_SIZE_BYTES + 1)
            .map(|_| rand::random::<u8>())
            .collect();
        let res = server
            .call(StoreMessage(StoreMessageRequest {
                user_message: Some(UserMessage {
                    net_id: 1,
                    created: Utc::now().timestamp() as u64,
                    address: address1,
                    transaction_type: TransactionType::VaultWithdraw as i32,
                    transaction_data: tx1,
                }),
            }))
            .await
            .unwrap();

        assert!(res.is_err());

        // cleanup
        let _ = server.call(DeleteDb {}).await.unwrap().unwrap();
    }

    #[tokio::test]
    #[serial]
    async fn reject_badly_timed_message() {
        setup_test();
        let server = Server::from_registry().await.unwrap();
        let _ = server
            .call(SetConfig(get_default_config()))
            .await
            .unwrap()
            .unwrap();
        let address1: Vec<u8> = (0..32).map(|_| rand::random::<u8>()).collect();
        let tx1: Vec<u8> = (0..1024).map(|_| rand::random::<u8>()).collect();

        // set time in past, before acceptable time window
        let t = Utc::now().timestamp() as u64 - (ACCEPTED_MESSAGES_TIME_WINDOW_SECS as u64) - 1;

        let res = server
            .call(StoreMessage(StoreMessageRequest {
                user_message: Some(UserMessage {
                    net_id: 1,
                    created: t,
                    address: address1.clone(),
                    transaction_type: TransactionType::VaultWithdraw as i32,
                    transaction_data: tx1.clone(),
                }),
            }))
            .await
            .unwrap();

        assert!(res.is_err());

        // set time in the future beyond acceptance window
        let t = Utc::now().timestamp() as u64 + (ACCEPTED_MESSAGES_TIME_WINDOW_SECS as u64) + 1;

        let res = server
            .call(StoreMessage(StoreMessageRequest {
                user_message: Some(UserMessage {
                    net_id: 1,
                    created: t,
                    address: address1.clone(),
                    transaction_type: TransactionType::VaultWithdraw as i32,
                    transaction_data: tx1.clone(),
                }),
            }))
            .await
            .unwrap();

        assert!(res.is_err());

        // cleanup
        let _ = server.call(DeleteDb {}).await.unwrap().unwrap();
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    #[serial]
    async fn reject_malicious_address() {
        setup_test();
        let server = Server::from_registry().await.unwrap();
        let _ = server
            .call(SetConfig(get_default_config()))
            .await
            .unwrap()
            .unwrap();
        let address1: Vec<u8> = Vec::from(ALL_ADDRESSES_KEY);
        let tx1: Vec<u8> = (0..1024).map(|_| rand::random::<u8>()).collect();
        let res = server
            .call(StoreMessage(StoreMessageRequest {
                user_message: Some(UserMessage {
                    net_id: 1,
                    created: Utc::now().timestamp() as u64,
                    address: address1,
                    transaction_type: TransactionType::VaultWithdraw as i32,
                    transaction_data: tx1,
                }),
            }))
            .await
            .unwrap();

        assert!(res.is_err());

        // cleanup
        let _ = server.call(DeleteDb {}).await.unwrap().unwrap();
    }
}
