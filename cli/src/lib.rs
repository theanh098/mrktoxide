pub mod cw721;
pub mod mrkt;
pub mod pallet;
pub mod shared;

use base64::{prelude::BASE64_STANDARD, Engine};
use database::DatabaseConnection;
use futures_util::StreamExt;
use service::CosmosClient;
use std::future::Future;
use tendermint_rpc::{
    dialect::v0_37::Event,
    event::{EventData, TxInfo},
    query::Query,
    SubscriptionClient, WebSocketClient,
};

static WSS_URL: &'static str = "wss://rpc.sei-apis.com/websocket?x-apikey=";

pub async fn listen_stream<'r, F, Fut>(
    ctx: StreamContext,
    query: Query,
    db: &'r DatabaseConnection,
    cosmos_client: &'r CosmosClient,
    tx_handler: F,
) where
    F: Fn(&'r DatabaseConnection, &'r CosmosClient, TxInfo) -> Fut,
    Fut: Future<Output = anyhow::Result<()>> + 'r,
{
    let (client, driver) = WebSocketClient::new(WSS_URL).await.unwrap();

    let driver_handle = tokio::spawn(async move { driver.run().await });

    let mut subs = client.subscribe(query).await.unwrap();

    while let Some(res) = subs.next().await {
        let event = res.unwrap();

        if let EventData::Tx { tx_result } = event.data {
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

    client.close().unwrap();

    // Await the driver's termination to ensure proper connection closure.
    let _ = driver_handle.await.unwrap();
}

pub fn to_wasm_payload(wasm_event: Event) -> Vec<WasmPayload> {
    wasm_event
        .attributes
        .into_iter()
        .map(|attribute| WasmPayload(to_utf8(attribute.key), to_utf8(attribute.value)))
        .collect::<Vec<WasmPayload>>()
}

pub fn find_attribute(payload: &[WasmPayload], key: &str) -> anyhow::Result<String> {
    payload
        .iter()
        .find(|WasmPayload(k, _)| k == key)
        .map(|WasmPayload(_, value)| value.to_string())
        .ok_or(anyhow::anyhow!(format!("missing attribute {}", key)))
}

#[derive(Debug)]
pub struct WasmPayload(String, String);

#[derive(Debug)]
pub enum StreamContext {
    Mrkt,
    Pallet,
    Cw721,
}

fn to_utf8(base64: String) -> String {
    let buffer = BASE64_STANDARD.decode(base64).unwrap_or_default();
    String::from_utf8(buffer).unwrap_or_default()
}
