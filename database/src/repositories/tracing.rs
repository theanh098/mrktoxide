use crate::entities::sea_orm_active_enums::StreamContext;
use crate::entities::stream_tx;
use crate::StreamTx;
use sea_orm::{prelude::DateTimeWithTimeZone, DatabaseConnection, EntityTrait};
use sea_orm::{ColumnTrait, DbErr, QueryFilter, Set};

pub async fn find_stream_tx_by_tx_hash(
    db: &DatabaseConnection,
    tx_hash: &str,
) -> Result<Option<stream_tx::Model>, DbErr> {
    StreamTx::find()
        .filter(stream_tx::Column::TxHash.eq(tx_hash))
        .one(db)
        .await
}

pub async fn create_stream_tx(
    db: &DatabaseConnection,
    params: CreateStreamTxParams,
) -> Result<(), DbErr> {
    let stream_tx = stream_tx::ActiveModel {
        tx_hash: Set(params.tx_hash),
        action: Set(params.action),
        context: Set(params.context),
        event: Set(params.event),
        date: Set(params.date),
        is_failure: Set(params.is_failure),
        message: Set(params.message),
        ..Default::default()
    };

    StreamTx::insert(stream_tx).exec(db).await?;

    Ok(())
}

pub struct CreateStreamTxParams {
    pub tx_hash: String,
    pub action: String,
    pub event: serde_json::Value,
    pub context: StreamContext,
    pub date: DateTimeWithTimeZone,
    pub is_failure: bool,
    pub message: Option<String>,
}
