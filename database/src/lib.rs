#![allow(unused_imports)]
#![allow(dead_code)]
mod entities;
use entities::prelude::*;

pub use entities::sea_orm_active_enums;
use enumscribe::ScribeStaticStr;
pub use sea_orm::*;
use serde::Deserialize;
pub mod repositories;

#[derive(ScribeStaticStr, Deserialize, Debug)]
pub enum Sort {
    #[enumscribe(str = "ASC")]
    #[serde(rename(deserialize = "asc"))]
    Asc,

    #[enumscribe(str = "DESC")]
    #[serde(rename(deserialize = "desc"))]
    Desc,
}

impl Default for Sort {
    fn default() -> Self {
        Self::Desc
    }
}
