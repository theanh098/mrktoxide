use cli::{cw721::tx_handler, stream_handler, RPC_URL};
use database::{ConnectOptions, Database};
use service::CosmosClient;
use tendermint_rpc::query::{EventType, Query};
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("db_url must be set");
    let cosmos_client = CosmosClient::from(tendermint_rpc::HttpClient::new(RPC_URL).unwrap());

    let mut opt = ConnectOptions::new(db_url);
    opt.sqlx_logging(false);

    let db = Database::connect(opt).await.unwrap();

    let query = Query::from(EventType::Tx)
        .and_exists("wasm.action")
        .and_exists("wasm._contract_address")
        .and_exists("wasm.token_id")
        .to_string();

    dbg!(&query);

    let msg = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "subscribe",
        "id": "0",
        "params": {
          "query": query
        }
    });

    let msg = Message::text(msg.to_string());

    loop {
        if let Err(error) = stream_handler(&db, &cosmos_client, &msg, tx_handler).await {
            eprintln!("{}", error)
        }
    }
}
