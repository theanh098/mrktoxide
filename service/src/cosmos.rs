use prost::{DecodeError, Message};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;
use tendermint_rpc::{Client, HttpClient};

pub struct CosmosClient(HttpClient);

#[derive(Deserialize, Debug)]
pub struct ContractInfo {
    pub name: String,
    pub symbol: String,
}

#[derive(Deserialize, Debug)]
pub struct Supply {
    pub count: u32,
}

#[derive(Deserialize, Debug)]
pub struct NftInfo {
    pub token_uri: String,
    pub extension: Option<Extension>,
}

#[derive(Deserialize, Debug)]
pub struct Extension {
    pub royalty_percentage: Option<f32>,
}

#[derive(Deserialize, Debug)]
pub struct NftOwner {
    pub owner: String,
}

#[derive(thiserror::Error, Debug)]
pub enum CosmosClientError {
    #[error("Rpc errors : {0}")]
    RpcError(String),

    #[error("Json Error")]
    JsonError(#[from] serde_json::Error),

    #[error("Tendermint RPC Error")]
    TendermintRpcError(#[from] tendermint_rpc::Error),

    #[error("Decode Error")]
    ProstDecodeError(#[from] DecodeError),
}

impl CosmosClient {
    pub fn from(http_client: HttpClient) -> Self {
        Self(http_client)
    }

    pub async fn get_cw721_contract_info(
        &self,
        address: &str,
    ) -> Result<ContractInfo, CosmosClientError> {
        let msg = json!({
            "contract_info": {}
        });

        self.query_contract(address, msg).await
    }

    pub async fn get_cw721_contract_supply(
        &self,
        address: &str,
    ) -> Result<Supply, CosmosClientError> {
        let msg = json!({
            "num_tokens": {}
        });

        self.query_contract(address, msg).await
    }

    fn inner(&self) -> &HttpClient {
        &self.0
    }

    pub async fn get_nft_info(
        &self,
        address: &str,
        token_id: &str,
    ) -> Result<NftInfo, CosmosClientError> {
        let msg = json!({
            "nft_info": {
                "token_id": token_id
            }
        });

        self.query_contract(address, msg).await
    }

    pub async fn get_nft_owner(
        &self,
        address: &str,
        token_id: &str,
    ) -> Result<NftOwner, CosmosClientError> {
        let msg = json!({
            "owner_of": {
                "token_id": token_id
            }
        });

        self.query_contract(address, msg).await
    }

    async fn query_contract<T, U>(&self, address: &str, msg: T) -> Result<U, CosmosClientError>
    where
        T: Serialize,
        U: DeserializeOwned,
    {
        let query = QueryContractRequest {
            address: address.to_string(),
            query_data: serde_json::to_vec(&msg)?,
        };

        let res = self
            .inner()
            .abci_query(
                Some("/cosmwasm.wasm.v1.Query/SmartContractState".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if res.code.is_err() {
            return Err(CosmosClientError::RpcError(res.log));
        }

        let raw = QueryRawContractResponse::decode(res.value.as_slice())?;

        let res = serde_json::from_slice::<U>(raw.data.as_slice())?;

        Ok(res)
    }
}

#[derive(Clone, PartialEq, prost::Message)]
struct QueryContractRequest {
    #[prost(string, tag = "1")]
    address: prost::alloc::string::String,

    #[prost(bytes = "vec", tag = "2")]
    query_data: prost::alloc::vec::Vec<u8>,
}

#[derive(Clone, PartialEq, prost::Message)]
struct QueryRawContractResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub data: prost::alloc::vec::Vec<u8>,
}
