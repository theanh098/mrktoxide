use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[tokio::main]
async fn main() {
    let (ws_stream, _) = connect_async("wss://rpc.sei-apis.com/websocket?x-apikey=06cf555f")
        .await
        .expect("fail to connect");

    let (mut write, mut read) = ws_stream.split();

    let msg = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "subscribe",
        "id": "0",
        "params": {
          "query": "tm.event = 'Tx' AND wasm._contract_address EXISTS AND wasm.token_id EXISTS AND wasm.action EXISTS"
        }
    });

    let msg = Message::text(msg.to_string());

    write.send(msg).await.expect("fail to send msg");

    while let Some(message) = read.next().await {
        println!("message: {:#?}", message);
    }
}
