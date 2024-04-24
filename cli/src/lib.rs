#![allow(unused_imports)]
#![allow(dead_code)]
pub mod cw721;
pub mod mrkt;
pub mod pallet;
pub mod shared;

use anyhow::{anyhow, bail};
use base64::{prelude::BASE64_STANDARD, Engine};
use database::{query, DatabaseConnection};
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use service::CosmosClient;
use std::future::Future;
use tendermint_rpc::query::Query;
use tokio_tungstenite::{connect_async, tungstenite::Message};

pub static RPC_URL: &'static str = "https://rpc.sei-apis.com?x-apikey=06cf555f";
static WSS_URL: &'static str = "wss://rpc.sei-apis.com/websocket?x-apikey=06cf555f";
static INGORE_MESSAGE: &'static str = "{\"jsonrpc\":\"2.0\",\"id\":\"0\",\"result\":{}}";

pub trait FromJsonValue
where
    Self: Sized,
{
    fn try_from_value(value: serde_json::Value) -> anyhow::Result<Self>;
}

#[derive(Debug)]
pub struct Transaction {
    pub tx_hash: String,
    pub events: Vec<Event>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Event {
    pub r#type: String,
    pub attributes: Vec<Attribute>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Attribute {
    pub key: String,
    pub value: String,
}

pub async fn stream_handler<'r, F, Fut>(
    db: &'r DatabaseConnection,
    cosmos_client: &'r CosmosClient,
    msg_subcribe: &Message,
    tx_handler: F,
) -> anyhow::Result<()>
where
    F: Fn(&'r DatabaseConnection, &'r CosmosClient, Transaction) -> Fut,
    Fut: Future<Output = ()> + 'r,
{
    let (ws_stream, _) = connect_async(WSS_URL).await?;

    let (mut write, mut read) = ws_stream.split();

    write.send(msg_subcribe.to_owned()).await?;

    while let Some(message) = read.next().await {
        if let Message::Text(message) = message? {
            if message != INGORE_MESSAGE {
                let tx_result = serde_json::from_str::<Value>(&message)
                    .map_err(|e| anyhow!("unxepected error can not parse raw msg, {}", e))
                    .and_then(<Transaction as FromJsonValue>::try_from_value)?;

                tx_handler(db, cosmos_client, tx_result).await
            } else {
                // we skip first message, so this time is perfect to tell that stream is working
                println!("listening stream")
            }
        }
    }

    Ok(())
}

pub fn find_attribute(event: &Event, key: &str) -> anyhow::Result<String> {
    event
        .attributes
        .iter()
        .find(|Attribute { key: k, .. }| k == key)
        .map(|attribute| attribute.value.to_owned())
        .ok_or(anyhow::anyhow!(format!("missing attribute {}", key)))
}

impl FromJsonValue for Transaction {
    fn try_from_value(value: serde_json::Value) -> anyhow::Result<Transaction> {
        let tx_hash = value
            .get("result")
            .and_then(|v| v.get("events"))
            .and_then(|v| v.get("tx.hash"))
            .and_then(|v| v.get(0))
            .ok_or_else(|| anyhow!("unexpected error missing tx.hash attribute"))?;

        let tx_hash = match tx_hash {
            Value::String(tx_hash) => tx_hash,
            _ => bail!("unexpected error missing result.events[tx.hash] is not string"),
        };

        let events = value
            .get("result")
            .and_then(|v| v.get("data"))
            .and_then(|v| v.get("value"))
            .and_then(|v| v.get("TxResult"))
            .and_then(|v| v.get("result"))
            .and_then(|v| v.get("events"))
            .ok_or_else(|| {
                anyhow!(
                    "unexpected error missing result.data.value.TxResult.result.events attribute"
                )
            })?;

        let events = serde_json::from_value::<Vec<Event>>(events.to_owned())?
            .into_iter()
            .map(|Event { attributes, r#type }| Event {
                r#type,
                attributes: attributes
                    .into_iter()
                    .map(|Attribute { key, value }| Attribute {
                        key: to_utf8(key),
                        value: to_utf8(value),
                    })
                    .collect(),
            })
            .collect();

        Ok(Transaction {
            tx_hash: tx_hash.to_owned(),
            events,
        })
    }
}

pub fn create_subcribe_message(query: Query) -> Message {
    let msg = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "subscribe",
        "id": "0",
        "params": {
          "query": query.to_string()
        }
    });

    let msg = Message::text(msg.to_string());

    msg
}

fn to_utf8(base64: String) -> String {
    let buffer = BASE64_STANDARD.decode(base64).unwrap_or_default();
    String::from_utf8(buffer).unwrap_or_default()
}
