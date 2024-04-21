use service::CosmosClient;

#[tokio::main]
async fn main() {
    // cli::listen_stream(ctx, query, tx_handler)

    let http_client =
        tendermint_rpc::HttpClient::new("https://rpc.sei-apis.com?x-apikey=").unwrap();

    let client = CosmosClient::from(http_client);

    let t = client
        .get_cw721_contract_info("sei1v90ly54qeu7497lzk2mnmp2h29sgtep8hs5ryvfqf8dwq5gc0t9srp6aey")
        .await
        .unwrap();

    println!("t: {:#?}", t);
}
