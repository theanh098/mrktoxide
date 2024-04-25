use crate::{error::AppError, extractors::AppState};
use axum::{
    extract::{Query, State},
    Json,
};
use database::{
    query,
    repositories::{
        self,
        collection::{CollectionStatSelectOption, CollectionWithStat},
    },
    Sort,
};
use serde::Deserialize;
use server::{empty_string_as_none, PagedQuery, PaginatedReponse};

pub async fn get_collections(
    State(AppState { db, .. }): State<AppState>,
    Query(query): Query<GetCollectionsQuery>,
    Query(paged_query): Query<PagedQuery>,
) -> Result<Json<PaginatedReponse<CollectionWithStat>>, AppError> {
    use CollectionStatSelectOption::*;

    let GetCollectionsQuery {
        search,
        sort_direction,
        sort_by,
    } = query;

    let PagedQuery { page, take } = paged_query;

    let sort_by = sort_by.map(|s| s.to_stat_field()).unwrap_or(Volume);

    let cols = [
        Address, Name, Image, Volume, FloorPrice, Sales, Listed, sort_by,
    ];

    let collections = repositories::collection::find_collections_with_stats(
        &db,
        cols,
        search,
        (Some(page), Some(take)),
        (sort_by, sort_direction.unwrap_or_default()),
    )
    .await?;

    Ok(Json(PaginatedReponse {
        page,
        total: 100,
        data: collections,
    }))
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
