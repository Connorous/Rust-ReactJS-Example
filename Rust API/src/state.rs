use chrono::naive::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: PgPool,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, PartialEq)]
pub struct User {
    pub id: i64,
    pub date_created: NaiveDate,
    pub username: String,
    pub email: String,
    pub password: String,
    pub name: String,
    pub user_type_id: i64,
}
