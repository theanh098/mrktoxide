use axum::{async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use chrono::Utc;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppResult};

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub exp: u32,
    pub address: String,
}

#[derive(Deserialize, Serialize)]
pub struct SubClaims {
    pub exp: u32,
    pub sub: String,
}

pub struct Guard(pub Claims);

#[async_trait]
impl<S> FromRequestParts<S> for Guard
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> AppResult<Self> {
        let access_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set.");

        let bearer = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::UnauthorizedError("Missing Authorization".into()))?;

        jsonwebtoken::decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(access_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|err| match err.kind() {
            ErrorKind::ExpiredSignature => AppError::UnauthorizedError("Expired token".into()),
            _ => AppError::UnauthorizedError("Invalid token".into()),
        })
        .map(|token_data| Self(token_data.claims))
    }
}

impl Claims {
    pub fn new(address: String, expired: chrono::Duration) -> Self {
        Self {
            address,
            exp: Utc::now().checked_add_signed(expired).unwrap().timestamp() as u32,
        }
    }
}

impl SubClaims {
    pub fn new(address: String, expired: chrono::Duration) -> Self {
        Self {
            sub: address,
            exp: Utc::now().checked_add_signed(expired).unwrap().timestamp() as u32,
        }
    }
}
