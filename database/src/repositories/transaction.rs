use crate::Transaction;
use crate::{entities::transaction, sea_orm_active_enums::Marketplace};
use sea_orm::prelude::{DateTimeUtc, Decimal};
use sea_orm::{DatabaseConnection, DatabaseTransaction, DbErr, EntityTrait, Set};

pub async fn create(
    db: &DatabaseTransaction,
    params: CreateTransactionParams,
) -> Result<&DatabaseTransaction, DbErr> {
    let transaction = transaction::ActiveModel {
        buyer_address: Set(params.buyer_address),
        collection_address: Set(params.collection_address),
        date: Set(params.created_date.into()),
        market: Set(params.marketplace),
        seller_address: Set(params.seller_address),
        txn_hash: Set(params.tx_hash),
        volume: Set(params.volume),
        ..Default::default()
    };

    Transaction::insert(transaction).exec(db).await?;

    Ok(db)
}

pub struct CreateTransactionParams {
    pub tx_hash: String,
    pub volume: Decimal,
    pub collection_address: String,
    pub buyer_address: String,
    pub seller_address: String,
    pub created_date: DateTimeUtc,
    pub marketplace: Marketplace,
}
