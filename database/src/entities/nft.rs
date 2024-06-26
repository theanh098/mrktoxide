//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "nft")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub token_address: String,
    pub token_id: String,
    pub name: Option<String>,
    pub token_uri: String,
    pub image: Option<String>,
    pub description: Option<String>,
    pub owner_address: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::collection::Entity",
        from = "Column::TokenAddress",
        to = "super::collection::Column::Address",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    Collection,
    #[sea_orm(has_many = "super::listing_nft::Entity")]
    ListingNft,
    #[sea_orm(has_many = "super::nft_activity::Entity")]
    NftActivity,
    #[sea_orm(has_many = "super::nft_offer::Entity")]
    NftOffer,
    #[sea_orm(has_many = "super::nft_trait::Entity")]
    NftTrait,
}

impl Related<super::collection::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Collection.def()
    }
}

impl Related<super::listing_nft::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ListingNft.def()
    }
}

impl Related<super::nft_activity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::NftActivity.def()
    }
}

impl Related<super::nft_offer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::NftOffer.def()
    }
}

impl Related<super::nft_trait::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::NftTrait.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
