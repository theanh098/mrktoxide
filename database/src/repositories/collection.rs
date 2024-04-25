use enumscribe::ScribeStaticStr;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{
    prelude::Decimal, sea_query::OnConflict, DatabaseConnection, DbErr, EntityTrait, Set,
};
use sea_orm::{query, ConnectionTrait, DatabaseBackend, FromQueryResult, Statement};
use serde::Serialize;
use service::CollectionMetadata;

use crate::entities::collection;
use crate::{Collection, Sort};

pub async fn find_by_address(
    db: &DatabaseConnection,
    address: &str,
) -> Result<Option<collection::Model>, DbErr> {
    Collection::find_by_id(address).one(db).await
}

pub async fn create(db: &DatabaseConnection, params: CreateCollectionParams) -> Result<(), DbErr> {
    let collection = collection::ActiveModel {
        address: Set(params.address),
        name: Set(params.name),
        symbol: Set(params.symbol),
        supply: Set(params.supply),
        description: Set(params.metadata.description),
        royalty: Set(params.royalty),
        banner: Set(params.metadata.banner),
        image: Set(params.metadata.pfp),
        socials: Set(if let Some(socials) = params.metadata.socials {
            Some(socials)
        } else {
            None
        }),
        ..Default::default()
    };

    Collection::insert(collection)
        .on_conflict(
            OnConflict::column(collection::Column::Address)
                .do_nothing()
                .to_owned(),
        )
        .exec(db)
        .await?;

    Ok(())
}

pub async fn find_collections_with_stats(
    db: &DatabaseConnection,
    cols: impl IntoIterator<Item = CollectionStatSelectOption>,
    search: Option<String>,
    (page, limit): (Option<u32>, Option<u16>),
    (col, sort): (CollectionStatSelectOption, Sort),
) -> Result<Vec<CollectionWithStat>, DbErr> {
    let cols = cols
        .into_iter()
        .map(|col| col.scribe())
        .collect::<Vec<&str>>()
        .join(",");

    let limit = limit.unwrap_or(100);
    let skip = (page.unwrap_or(1) - 1) * limit as u32;
    let col = col.scribe();
    let sort = sort.scribe();
    let search = search
        .map(|s| format!("LOWER(name) LIKE '%{}%'", s.to_lowercase()))
        .unwrap_or("1 = 1".to_owned());

    let collections = query::JsonValue::find_by_statement(Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        format!(
            "SELECT {} FROM collection_view
            WHERE {}
            ORDER BY {} {} NULLS LAST 
            OFFSET {} LIMIT {};",
            cols, search, col, sort, skip, limit
        ),
        [],
    ))
    .into_model::<CollectionWithStat>()
    .all(db)
    .await?;

    Ok(collections)
}

pub struct CreateCollectionParams {
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub supply: i32,
    pub metadata: CollectionMetadata,
    pub royalty: Option<Decimal>,
}

#[derive(ScribeStaticStr, Clone, Copy)]
pub enum CollectionStatSelectOption {
    #[enumscribe(str = "address")]
    Address,

    #[enumscribe(str = "name")]
    Name,

    #[enumscribe(str = "symbol")]
    Symbol,

    #[enumscribe(str = "sales")]
    Sales,

    #[enumscribe(str = "volume")]
    Volume,

    #[enumscribe(str = "royalty")]
    Royalty,

    #[enumscribe(str = "image")]
    Image,

    #[enumscribe(str = "banner")]
    Banner,

    #[enumscribe(str = "description")]
    Description,

    #[enumscribe(str = "socials")]
    Socials,

    #[enumscribe(str = "supply")]
    Supply,

    #[enumscribe(str = "highest_bid")]
    HighestBid,

    #[enumscribe(str = "listed")]
    Listed,

    #[enumscribe(str = "minted_date")]
    MintedDate,

    #[enumscribe(str = "floor_price")]
    FloorPrice,

    #[enumscribe(str = "volume_of_1h")]
    VolumeOf1h,

    #[enumscribe(str = "volume_of_24h")]
    VolumeOf24h,

    #[enumscribe(str = "volume_of_7d")]
    VolumeOf7d,

    #[enumscribe(str = "volume_of_30d")]
    VolumeOf30d,
}

#[derive(Serialize, FromQueryResult, Debug)]
pub struct CollectionWithStat {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub supply: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub royalty: Option<Decimal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub socials: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub highest_bid: Option<Decimal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub listed: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minted_date: Option<DateTimeWithTimeZone>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<Decimal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor_price: Option<Decimal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_of_1h: Option<Decimal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_of_24h: Option<Decimal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_of_7d: Option<Decimal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_of_30d: Option<Decimal>,
}
