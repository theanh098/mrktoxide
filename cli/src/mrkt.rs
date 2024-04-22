use anyhow::{anyhow, bail};
use database::DatabaseConnection;
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use service::CosmosClient;
use std::future::Future;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::StreamContext;

static MRKT_CONTRACT_ADDRESS: &'static str =
    "sei152u2u0lqc27428cuf8dx48k8saua74m6nql5kgvsu4rfeqm547rsnhy4y9";
static WSS_URL: &'static str = "wss://rpc.sei-apis.com/websocket";
static INGORE_MESSAGE: &'static str = "{\"jsonrpc\":\"2.0\",\"id\":\"0\",\"result\":{}}";

#[derive(Debug)]
pub struct TxResult {
    pub tx_hash: String,
    pub events: Vec<Event>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Event {
    pub r#type: String,
    pub attributes: Vec<Attribute>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Attribute {
    pub key: String,
    pub value: String,
}

pub trait FromJsonValue
where
    Self: Sized,
{
    fn try_from_value(value: serde_json::Value) -> anyhow::Result<Self>;
}

impl FromJsonValue for TxResult {
    fn try_from_value(value: serde_json::Value) -> anyhow::Result<TxResult> {
        let tx_hash = value
            .get("result")
            .and_then(|v| v.get("events"))
            .and_then(|v| v.get("tx.hash"))
            .and_then(|v| v.get(0))
            .ok_or_else(|| anyhow!("unexpected missing tx.hash attribute"))?;

        let tx_hash = match tx_hash {
            Value::String(tx_hash) => tx_hash,
            _ => bail!("unexpected result.events[tx.hash] is not string"),
        };

        let events = value
            .get("result")
            .and_then(|v| v.get("data"))
            .and_then(|v| v.get("value"))
            .and_then(|v| v.get("TxResult"))
            .and_then(|v| v.get("result"))
            .and_then(|v| v.get("events"))
            .ok_or_else(|| {
                anyhow!("unexpected missing result.data.value.TxResult.result.events attribute")
            })?;

        let events: Vec<Event> = serde_json::from_value(events.to_owned())?;

        Ok(TxResult {
            tx_hash: tx_hash.to_owned(),
            events,
        })
    }
}

pub async fn stream_handler<'r, F, Fut>(
    db: &'r DatabaseConnection,
    cosmos_client: &'r CosmosClient,
    ctx: StreamContext,
    msg_subcribe: Message,
    tx_handler: F,
) where
    F: Fn(&'r DatabaseConnection, &'r CosmosClient, TxResult) -> Fut,
    Fut: Future<Output = anyhow::Result<()>> + 'r,
{
    let (ws_stream, _) = connect_async(WSS_URL).await.expect("failed to connect ws");

    let (mut write, mut read) = ws_stream.split();

    let msg = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "subscribe",
        "id": "0",
        "params": {
          "query": format!("tm.event = 'Tx'
                                AND wasm._contract_address EXISTS 
                                AND wasm.token_id EXISTS 
                                AND wasm.action EXISTS")
        }
    });

    let msg = Message::text(msg.to_string());

    write.send(msg).await.expect("fail to send msg");

    while let Some(message) = read.next().await {
        match message {
            Err(err) => {
                eprintln!("unxepected error when read ws message {}", err);
            }
            Ok(message) => {
                if let Message::Text(message) = message {
                    if message != INGORE_MESSAGE {
                        let tx_result = serde_json::from_str::<Value>(&message)
                            .map_err(|e| anyhow!("unxepected error can not parse raw msg, {}", e))
                            .and_then(<TxResult as FromJsonValue>::try_from_value);

                        match tx_result {
                            Err(e) => eprintln!("{}", e),
                            Ok(tx_result) => {
                                tx_handler(db, cosmos_client, tx_result)
                                    .await
                                    .unwrap_or_else(|error| {
                                        eprintln!(
                                            "{:#?}>> Unxpected error when hanlde tx event: {:#?}",
                                            ctx, error
                                        );
                                    });
                            }
                        }
                    }
                }
            }
        }
    }
}
