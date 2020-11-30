use chrono::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Expense {
    pub id: i32,
    pub account_id: i32,
    pub amount: i64,
    pub category: String,
    pub created_at: DateTime<Utc>,
    pub merchant: String,
    pub notes: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpenseRequest {
    pub account_id: i32,
    pub amount: i64,
    pub category: String,
    pub merchant: String,
    pub notes: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpenseUpdateRequest {
    pub account_id: i32,
    pub amount: i64,
    pub category: String,
    pub merchant: String,
    pub notes: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpenseResponse {
    pub id: i32,
    pub account_id: i32,
    pub amount: i64,
    pub category: String,
    pub created_at: DateTime<Utc>,
    pub merchant: String,
    pub notes: String,
}

impl ExpenseResponse {
    pub fn of(expense: Expense) -> ExpenseResponse {
        ExpenseResponse {
            id: expense.id,
            account_id: expense.account_id,
            amount: expense.amount,
            category: expense.category,
            created_at: expense.created_at,
            merchant: expense.merchant,
            notes: expense.notes,
        }
    }
}
