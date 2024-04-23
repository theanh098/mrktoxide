use crate::UserLoyaltyPoint;
use crate::{entities::user_loyalty_point, sea_orm_active_enums::LoyaltyPointKind};
use sea_orm::{prelude::DateTimeUtc, DbErr};
use sea_orm::{DatabaseTransaction, EntityTrait, Set};

pub async fn create(db: &DatabaseTransaction, params: CreateUserPointParams) -> Result<(), DbErr> {
    let user_point = user_loyalty_point::ActiveModel {
        date: Set(params.date.into()),
        kind: Set(params.kind),
        point: Set(params.point),
        wallet_address: Set(params.wallet_address),
        ..Default::default()
    };

    UserLoyaltyPoint::insert(user_point).exec(db).await?;

    Ok(())
}

pub struct CreateUserPointParams {
    pub date: DateTimeUtc,
    pub kind: LoyaltyPointKind,
    pub wallet_address: String,
    pub point: i32,
}
