use crate::{AppResult, errors::*};
use axum_responses::HttpResult;

async fn service() -> AppResult<()> {
    Err(SimpleError::InvalidRequest(vec![]))?
}

async fn service_2() -> AppResult<()> {
    Err(AppError::DatabaseError(sqlx::Error::RowNotFound))
}

pub async fn handler() -> HttpResult {
    let details = vec![
        Detail {
            field: "file".into(),
            issue: "missing".into(),
        },
        Detail {
            field: "type".into(),
            issue: "unsupported".into(),
        },
    ];

    let service_data = service().await?;
    let service_2_data = service_2().await?;

    Err(SimpleError::InvalidRequest(details))?
}
