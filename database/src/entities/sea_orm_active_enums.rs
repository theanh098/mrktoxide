//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "loyalty_point_kind")]
pub enum LoyaltyPointKind {
    #[sea_orm(string_value = "bid")]
    Bid,
    #[sea_orm(string_value = "buy")]
    Buy,
    #[sea_orm(string_value = "sell")]
    Sell,
    #[sea_orm(string_value = "xp")]
    Xp,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "marketplace")]
pub enum Marketplace {
    #[sea_orm(string_value = "mrkt")]
    Mrkt,
    #[sea_orm(string_value = "pallet")]
    Pallet,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "nft_activity_kind")]
pub enum NftActivityKind {
    #[sea_orm(string_value = "cancel_offer")]
    CancelOffer,
    #[sea_orm(string_value = "delist")]
    Delist,
    #[sea_orm(string_value = "list")]
    List,
    #[sea_orm(string_value = "make_offer")]
    MakeOffer,
    #[sea_orm(string_value = "sale")]
    Sale,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "sale_type")]
pub enum SaleType {
    #[sea_orm(string_value = "auction")]
    Auction,
    #[sea_orm(string_value = "fixed")]
    Fixed,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "stream_context")]
pub enum StreamContext {
    #[sea_orm(string_value = "cwr721")]
    Cwr721,
    #[sea_orm(string_value = "launchpad")]
    Launchpad,
    #[sea_orm(string_value = "mrkt")]
    Mrkt,
    #[sea_orm(string_value = "pallet")]
    Pallet,
}
