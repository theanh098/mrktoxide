use cli::RPC_URL;
use service::CosmosClient;
use tendermint_rpc::query::Query;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let cosmos_client = CosmosClient::from(tendermint_rpc::HttpClient::new(RPC_URL).unwrap());

    let query = Query::eq("tx.height", "71718504");

    let res = cosmos_client.search_tx(query, 1, 100).await.unwrap();

    println!("total tx {}", res.total_count)
}
