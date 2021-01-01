use crate::model::expense::{ExpenseRequest, ExpenseResponse, ExpenseUpdateRequest};
use crate::service::expense_service;
use crate::{data::*, db, error::Error::*, DBPool, Result};
use serde_derive::Deserialize;
use warp::{http::StatusCode, reject, reply::json, Reply};

#[derive(Deserialize)]
pub struct SearchQuery {
    search: Option<String>,
    limit: Option<String>,
    offset: Option<String>,
}

pub async fn list_expense_handler(query: SearchQuery, db_pool: DBPool) -> Result<impl Reply> {
    let expenses = expense_service::fetch_expenses(&db_pool, query.limit, query.offset)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(json::<Vec<_>>(
        &expenses
            .into_iter()
            .map(|t| ExpenseResponse::of(t))
            .collect(),
    ))
}

pub async fn create_expense_handler(body: ExpenseRequest, db_pool: DBPool) -> Result<impl Reply> {
    Ok(json(&ExpenseResponse::of(
        expense_service::create_expense(&db_pool, body)
            .await
            .map_err(|e| reject::custom(e))?,
    )))
}

pub async fn update_expense_handler(
    id: i32,
    body: ExpenseUpdateRequest,
    db_pool: DBPool,
) -> Result<impl Reply> {
    Ok(json(&ExpenseResponse::of(
        expense_service::update_expense(&db_pool, id, body)
            .await
            .map_err(|e| reject::custom(e))?,
    )))
}

pub async fn delete_expense_handler(id: i32, db_pool: DBPool) -> Result<impl Reply> {
    expense_service::delete_expense(&db_pool, id)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}
