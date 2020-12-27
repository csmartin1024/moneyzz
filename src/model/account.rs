use chrono::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountRequest {
    pub name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountUpdateRequest {
    pub name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountResponse {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

impl AccountResponse {
    pub fn of(account: Account) -> AccountResponse {
        AccountResponse {
            id: account.id,
            name: account.name,
            created_at: account.created_at,
        }
    }
}
