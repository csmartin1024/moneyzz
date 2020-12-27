use crate::model::account::{AccountRequest, AccountResponse, AccountUpdateRequest};
use crate::service::account_service;
use crate::{data::*, db, error::Error::*, DBPool, Result};
use serde_derive::Deserialize;
use warp::{http::StatusCode, reject, reply::json, Reply};

#[derive(Deserialize)]
pub struct SearchQuery {
    search: Option<String>,
}

pub async fn list_account_handler(query: SearchQuery, db_pool: DBPool) -> Result<impl Reply> {
    let accounts = account_service::fetch_accounts(&db_pool, query.search)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(json::<Vec<_>>(
        &accounts
            .into_iter()
            .map(|t| AccountResponse::of(t))
            .collect(),
    ))
}

pub async fn create_account_handler(body: AccountRequest, db_pool: DBPool) -> Result<impl Reply> {
    Ok(json(&AccountResponse::of(
        account_service::create_account(&db_pool, body)
            .await
            .map_err(|e| reject::custom(e))?,
    )))
}

pub async fn update_account_handler(
    id: i32,
    body: AccountUpdateRequest,
    db_pool: DBPool,
) -> Result<impl Reply> {
    Ok(json(&AccountResponse::of(
        account_service::update_account(&db_pool, id, body)
            .await
            .map_err(|e| reject::custom(e))?,
    )))
}

pub async fn delete_account_handler(id: i32, db_pool: DBPool) -> Result<impl Reply> {
    account_service::delete_account(&db_pool, id)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}
