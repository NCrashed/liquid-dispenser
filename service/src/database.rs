use diesel::{self, prelude::*, result::QueryResult};
use rocket::serde::Serialize;
use chrono::NaiveDateTime;
use thiserror::Error;

#[database("transactions")]
pub struct TransactionsDb(diesel::SqliteConnection);

// use super::schema::*;
// use super::schema::transactions::dsl::{transactions as all_transactions};
// use super::schema::transaction_updates::dsl::{transaction_updates as all_transaction_updates};
