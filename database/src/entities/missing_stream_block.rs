//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.11

use super::sea_orm_active_enums::StreamContext;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "missing_stream_block")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub height: String,
    pub context: StreamContext,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
