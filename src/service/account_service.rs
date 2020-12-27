use crate::model::account::{Account, AccountRequest, AccountUpdateRequest};
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
const TABLE: &str = "account";
const SELECT_FIELDS: &str = "id, name, created_at";

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

pub async fn fetch_accounts(db_pool: &DBPool, search: Option<String>) -> Result<Vec<Account>> {
    let con = get_db_con(db_pool).await?;
    let query = format!(
        "SELECT {} FROM {} ORDER BY created_at DESC",
        SELECT_FIELDS, TABLE
    );
    let q = match search {
        Some(v) => con.query(query.as_str(), &[&v]).await,
        None => con.query(query.as_str(), &[]).await,
    };
    let rows = q.map_err(DBQueryError)?;

    Ok(rows.iter().map(|r| row_to_account(&r)).collect())
}

pub async fn create_account(db_pool: &DBPool, body: AccountRequest) -> Result<Account> {
    let con = get_db_con(db_pool).await?;
    let query = format!(
        "INSERT INTO {} (name) VALUES ($1) RETURNING {}",
        TABLE, SELECT_FIELDS
    );
    let row = con
        .query_one(query.as_str(), &[&body.name])
        .await
        .map_err(DBQueryError)?;
    Ok(row_to_account(&row))
}

pub async fn update_account(
    db_pool: &DBPool,
    id: i32,
    body: AccountUpdateRequest,
) -> Result<Account> {
    let con = get_db_con(db_pool).await?;
    let query = format!(
        "UPDATE {} SET name = $1 WHERE id = $6 RETURNING {}",
        TABLE, SELECT_FIELDS
    );
    let row = con
        .query_one(query.as_str(), &[&body.name, &id])
        .await
        .map_err(DBQueryError)?;
    Ok(row_to_account(&row))
}

pub async fn delete_account(db_pool: &DBPool, id: i32) -> Result<u64> {
    let con = get_db_con(db_pool).await?;
    let query = format!("DELETE FROM {} WHERE id = $1", TABLE);
    con.execute(query.as_str(), &[&id])
        .await
        .map_err(DBQueryError)
}

fn row_to_account(row: &Row) -> Account {
    let id: i32 = row.get(0);
    let name: String = row.get(1);
    let created_at: DateTime<Utc> = row.get(2);
    Account {
        id,
        name,
        created_at,
    }
}
