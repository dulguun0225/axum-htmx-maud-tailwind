use std::time;
use sqlx::postgres::{PgPoolOptions, PgRow, types::*};
use sqlx::{FromRow, Row, Decode, };

#[derive(Debug, FromRow, Clone)]
pub struct Todo {
    id: i64,
    title: String,
    done: bool,
    // created_at: time::SystemTime,
    // updated_at: time::SystemTime,
}
