use serde::{Deserialize, Deserializer};
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
pub struct NftMetadata {
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Attribute {
    pub trait_type: Option<String>,
    pub r#type: Option<String>,
    pub value: Option<String>,
    #[serde(deserialize_with = "deserialize_display_type")]
    pub display_type: Option<String>,
}

fn deserialize_display_type<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<Value> = Deserialize::deserialize(deserializer).ok();

    let Some(value) = value else {
        return Ok(None);
    };

    match value {
        Value::String(s) => Ok(Some(s)),
        Value::Number(n) => Ok(Some(n.to_string())),
        _ => Ok(None),
    }
}

pub async fn get_nft_metadata(uri: &str) -> Result<NftMetadata, reqwest::Error> {
    reqwest::get(uri).await?.json::<NftMetadata>().await
}
