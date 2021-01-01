use crate::model::expense::{Expense, ExpenseRequest, ExpenseUpdateRequest};
use crate::{data::*, error, error::Error::*, DBCon, DBPool};
use chrono::prelude::*;
use mobc::Pool;
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use std::env;
use std::str::FromStr;
use std::time::Duration;
use tokio_postgres::{Config, Error, NoTls, Row};

type Result<T> = std::result::Result<T, error::Error>;

const DB_POOL_MAX_OPEN: u64 = 32;
const DB_POOL_MAX_IDLE: u64 = 8;
const DB_POOL_TIMEOUT_SECONDS: u64 = 15;
const TABLE: &str = "expense";
const LIMIT: &str = "25";
const OFFSET: &str = "0";
const SELECT_FIELDS: &str = "id, account_id, amount, category, merchant, notes, created_at";

pub async fn get_db_con(db_pool: &DBPool) -> Result<DBCon> {
    db_pool.get().await.map_err(DBPoolError)
}

pub fn create_pool() -> std::result::Result<DBPool, mobc::Error<Error>> {
    let db_url = env::var("DATABASE_URL").expect("Database url not set");
    // let config = Config::from_str("postgres://postgres@127.0.0.1:7878/postgres")?;
    let config = Config::from_str(&db_url)?;

    let manager = PgConnectionManager::new(config, NoTls);
    Ok(Pool::builder()
        .max_open(DB_POOL_MAX_OPEN)
        .max_idle(DB_POOL_MAX_IDLE)
        .get_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)))
        .build(manager))
}

pub async fn fetch_expenses(
    db_pool: &DBPool,
    limit_param: Option<String>,
    offset_param: Option<String>,
) -> Result<Vec<Expense>> {
    let con = get_db_con(db_pool).await?;
    let limit = match limit_param {
        Some(v) => v,
        None => LIMIT.to_string(),
    };
    let offset = match offset_param {
        Some(v) => v,
        None => OFFSET.to_string(),
    };
    let query = format!(
        "SELECT {} FROM {} ORDER BY created_at DESC OFFSET {} LIMIT {}",
        SELECT_FIELDS, TABLE, offset, limit
    );
    let q = con.query(query.as_str(), &[]).await;
    let rows = q.map_err(DBQueryError)?;

    Ok(rows.iter().map(|r| row_to_expense(&r)).collect())
}

pub async fn create_expense(db_pool: &DBPool, body: ExpenseRequest) -> Result<Expense> {
    let con = get_db_con(db_pool).await?;
    let query = format!(
        "INSERT INTO {} (account_id,amount,category,merchant,notes) VALUES ($1,$2,$3,$4,$5) RETURNING {}",
        TABLE,
        SELECT_FIELDS
    );
    let row = con
        .query_one(
            query.as_str(),
            &[
                &body.account_id,
                &body.amount,
                &body.category,
                &body.merchant,
                &body.notes,
            ],
        )
        .await
        .map_err(DBQueryError)?;
    Ok(row_to_expense(&row))
}

pub async fn update_expense(
    db_pool: &DBPool,
    id: i32,
    body: ExpenseUpdateRequest,
) -> Result<Expense> {
    let con = get_db_con(db_pool).await?;
    let query = format!(
        "UPDATE {} SET account_id = $1, amount = $2, category = $3, merchant = $4, notes = $5 WHERE id = $6 RETURNING {}",
        TABLE,
        SELECT_FIELDS
    );
    let row = con
        .query_one(
            query.as_str(),
            &[
                &body.account_id,
                &body.amount,
                &body.category,
                &body.merchant,
                &body.notes,
                &id,
            ],
        )
        .await
        .map_err(DBQueryError)?;
    println!("Got here");
    Ok(row_to_expense(&row))
}

pub async fn delete_expense(db_pool: &DBPool, id: i32) -> Result<u64> {
    let con = get_db_con(db_pool).await?;
    let query = format!("DELETE FROM {} WHERE id = $1", TABLE);
    con.execute(query.as_str(), &[&id])
        .await
        .map_err(DBQueryError)
}

fn row_to_expense(row: &Row) -> Expense {
    let id: i32 = row.get(0);
    let account_id: i32 = row.get(1);
    let amount: i64 = row.get(2);
    let category: String = row.get(3);
    let merchant: String = row.get(4);
    let notes: String = row.get(5);
    let created_at: DateTime<Utc> = row.get(6);
    Expense {
        id,
        account_id,
        amount,
        category,
        created_at,
        merchant,
        notes,
    }
}
