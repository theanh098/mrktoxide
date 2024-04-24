use axum::extract::Query;
use database::{query, repositories::collection::CollectionStatSelectOption, Sort};
use serde::Deserialize;
use server::empty_string_as_none;

pub async fn get_collections(query: Query<GetCollectionsQuery>) {
    println!("query: {:#?}", query);
}

#[derive(Deserialize, Debug)]
pub struct GetCollectionsQuery {
    #[serde(deserialize_with = "empty_string_as_none")]
    search: Option<String>,

    sort_direction: Option<Sort>,
    sort_by: Option<SortBy>,
}

#[derive(Deserialize, Debug)]
enum SortBy {
    #[serde(rename(deserialize = "1h"))]
    _1h,

    #[serde(rename(deserialize = "24h"))]
    _24h,

    #[serde(rename(deserialize = "30d"))]
    _30d,

    #[serde(rename(deserialize = "all"))]
    All,
}

impl SortBy {
    fn to_stat_field(&self) -> CollectionStatSelectOption {
        match self {
            Self::All => CollectionStatSelectOption::Volume,
            Self::_1h => CollectionStatSelectOption::VolumeOf1h,
            Self::_24h => CollectionStatSelectOption::VolumeOf24h,
            Self::_30d => CollectionStatSelectOption::VolumeOf30d,
        }
    }
}

impl Default for SortBy {
    fn default() -> Self {
        Self::_24h
    }
}
