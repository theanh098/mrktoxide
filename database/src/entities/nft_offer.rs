//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "nft_offer")]
pub struct Model {
    pub tx_hash: String,
    pub created_date: DateTimeWithTimeZone,
    pub nft_id: i32,
    #[sea_orm(column_type = "Decimal(Some((65, 30)))")]
    pub price: Decimal,
    pub buyer_address: String,
    pub start_date: DateTimeWithTimeZone,
    pub end_date: DateTimeWithTimeZone,
    pub denom: String,
    #[sea_orm(primary_key)]
    pub id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::nft::Entity",
        from = "Column::NftId",
        to = "super::nft::Column::Id",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    Nft,
}

impl Related<super::nft::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Nft.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
