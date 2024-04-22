use cli::{cw721::tx_handler, stream_handler, RPC_URL};
use database::{ConnectOptions, Database};
use service::CosmosClient;
use tendermint_rpc::query::{EventType, Query};
use tokio_tungstenite::tungstenite::Message;

static PALLET_CONTRACT_ADDRESS: &'static str =
    "sei152u2u0lqc27428cuf8dx48k8saua74m6nql5kgvsu4rfeqm547rsnhy4y9";

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("db_url must be set");
    let cosmos_client = CosmosClient::from(tendermint_rpc::HttpClient::new(RPC_URL).unwrap());

    let mut opt = ConnectOptions::new(db_url);
    opt.sqlx_logging(false);

    let db = Database::connect(opt).await.unwrap();

    let query = Query::from(EventType::Tx)
        .and_eq("execute._contract_address", PALLET_CONTRACT_ADDRESS)
        .to_string();

    let msg = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "subscribe",
        "id": "0",
        "params": {
          "query": query
        }
    });

    let msg = Message::text(msg.to_string());

    stream_handler(&db, &cosmos_client, &msg, tx_handler)
        .await
        .unwrap();
}
