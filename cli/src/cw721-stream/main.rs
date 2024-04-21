use cli::{cw721::tx_handler, listen_stream, StreamContext};
use database::{ConnectOptions, Database};
use service::CosmosClient;
use tendermint_rpc::query::{EventType, Query};

static RPC_URL: &'static str = "https://rpc.sei-apis.com?x-apikey=";

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
        .and_exists("wasm.token_id");

    listen_stream(StreamContext::Cw721, query, &db, &cosmos_client, tx_handler).await;
}
