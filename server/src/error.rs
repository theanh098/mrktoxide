use axum::{
    extract::rejection::{FormRejection, JsonRejection, QueryRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use database::error::DbErr;
use deadpool_redis::PoolError;
use serde_json::json;
use service::ServiceError;

pub type AppResult<A> = std::result::Result<A, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("{0}")]
    UnauthorizedError(String),

    #[error("{0}")]
    BadRequestError(String),

    #[error("{0}")]
    InternalError(String),

    #[error(transparent)]
    ServiceError(#[from] ServiceError),

    #[error(transparent)]
    DatabaseError(#[from] DbErr),

    #[error(transparent)]
    RedisPoolError(#[from] PoolError),

    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),

    #[error(transparent)]
    AxumQueryRejection(#[from] QueryRejection),

    #[error(transparent)]
    AxumPayloadRejection(#[from] JsonRejection),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::UnauthorizedError(reason) => (
                StatusCode::UNAUTHORIZED,
                to_json(StatusCode::UNAUTHORIZED, reason),
            ),
            AppError::BadRequestError(reason) => (
                StatusCode::BAD_REQUEST,
                to_json(StatusCode::BAD_REQUEST, reason),
            ),
            AppError::InternalError(reason) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                to_json(StatusCode::INTERNAL_SERVER_ERROR, reason),
            ),
            AppError::ServiceError(http_error) => {
                let status_code = http_error.status().unwrap_or_default();
                (
                    status_code,
                    to_json(
                        status_code,
                        format!(
                            "Error occured when sending http request, reason: {}",
                            http_error.to_string()
                        ),
                    ),
                )
            }
            AppError::DatabaseError(db_error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                to_json(StatusCode::INTERNAL_SERVER_ERROR, db_error.to_string()),
            ),
            AppError::RedisPoolError(pool_error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                to_json(StatusCode::INTERNAL_SERVER_ERROR, pool_error.to_string()),
            ),
            AppError::ValidationError(_) => {
                let message = format!("Input validation error: [{self}]").replace('\n', ", ");
                (
                    StatusCode::BAD_REQUEST,
                    to_json(StatusCode::BAD_REQUEST, message),
                )
            }
            _ => (
                StatusCode::BAD_REQUEST,
                to_json(StatusCode::BAD_REQUEST, self.to_string()),
            ),
        }
        .into_response()
    }
}

fn to_json(code: StatusCode, message: String) -> Json<serde_json::Value> {
    Json(json!({
        "code": code.as_u16(),
        "message": message,
        "status": code.to_string()
    }))
}
