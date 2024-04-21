use sea_orm::{
    prelude::Decimal, sea_query::OnConflict, DatabaseConnection, DbErr, EntityTrait, Set,
};

use crate::entities::collection;
use crate::Collection;

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
        description: Set(params.description),
        royalty: Set(params.royalty),
        banner: Set(params.banner),
        image: Set(params.image),
        socials: Set(if let Some(socials) = params.socials {
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

pub struct CreateCollectionParams {
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub supply: i32,
    pub description: Option<String>,
    pub royalty: Option<Decimal>,
    pub image: Option<String>,
    pub banner: Option<String>,
    pub socials: Option<serde_json::Value>,
}
