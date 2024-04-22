use sea_orm::prelude::{DateTimeWithTimeZone, Decimal};
use sea_orm::{
    sea_query::OnConflict, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
    TransactionTrait,
};

use crate::entities::{nft, nft_trait};
use crate::{Nft, NftTrait};

pub async fn find_by_address_and_token_id(
    db: &DatabaseConnection,
    token_address: &str,
    token_id: &str,
) -> Result<Option<nft::Model>, DbErr> {
    Nft::find()
        .filter(nft::Column::TokenAddress.eq(token_address))
        .filter(nft::Column::TokenId.eq(token_id))
        .one(db)
        .await
}

pub async fn update_owner(
    db: &DatabaseConnection,
    token_address: &str,
    token_id: &str,
    owner: Option<String>,
) -> Result<(), DbErr> {
    let nft = nft::ActiveModel {
        owner_address: Set(owner),
        ..Default::default()
    };

    Nft::update_many()
        .set(nft)
        .filter(nft::Column::TokenAddress.eq(token_address))
        .filter(nft::Column::TokenId.eq(token_id))
        .exec(db)
        .await?;

    Ok(())
}

pub async fn create(db: &DatabaseConnection, params: CreateNftParams) -> Result<i32, DbErr> {
    let txn = db.begin().await?;

    let nft = nft::ActiveModel {
        token_address: Set(params.token_address),
        token_id: Set(params.token_id),
        token_uri: Set(params.token_uri),
        description: Set(params.description),
        name: Set(params.name),
        owner_address: Set(params.owner_address),
        image: Set(params.image),
        ..Default::default()
    };

    let nft_id = Nft::insert(nft)
        .on_conflict(
            OnConflict::columns([nft::Column::TokenAddress, nft::Column::TokenId])
                .do_nothing()
                .to_owned(),
        )
        .exec(db)
        .await?
        .last_insert_id;

    let traits = params.traits.unwrap_or_default().into_iter().map(
        |AttributeParams {
             trait_type,
             r#type,
             value,
             display_type,
         }| nft_trait::ActiveModel {
            nft_id: Set(nft_id),
            attribute: Set(trait_type.unwrap_or(r#type.unwrap_or("unknown".to_string()))),
            display_type: Set(display_type),
            value: Set(value.unwrap_or("unknown".to_string())),
            ..Default::default()
        },
    );

    NftTrait::insert_many(traits)
        .on_empty_do_nothing()
        .exec(db)
        .await?;

    txn.commit().await?;

    Ok(nft_id)
}

pub async fn create_listing(db: &DatabaseConnection) {}

pub struct CreateNftParams {
    pub token_address: String,
    pub token_id: String,
    pub token_uri: String,
    pub name: Option<String>,
    pub image: Option<String>,
    pub traits: Option<Vec<AttributeParams>>,
    pub description: Option<String>,
    pub owner_address: Option<String>,
}

pub struct AttributeParams {
    pub trait_type: Option<String>,
    pub r#type: Option<String>,
    pub value: Option<String>,
    pub display_type: Option<String>,
}

pub struct CreatePalletListingParams {
    pub nft_id: i32,
    pub tx_hash: String,
    pub denom: String,
    pub amount: Decimal,
    pub pallet_listing: PalletListing,
}

pub struct PalletListing {
    nft_address: String,
    nft_token_id: String,
    owner: String,
    auction: Option<PalletAuction>,
}

pub struct PalletAuction {}
