pub mod todo;

use sqlx::{postgres::PgPoolOptions, query, Pool, Postgres};


pub static DB_STRING: &str = "postgres://postgres:postgres@localhost/todos_db";

pub async fn init() -> Pool<Postgres> {
    let result = PgPoolOptions::new()
        .max_connections(8)
        .connect(DB_STRING)
        .await;

    let pool = match result {
        Ok(pool) => pool,
        Err(e) => {
            tracing::error!("{}", e.to_string());
            panic!("Cannot connect to database {DB_STRING}");
        }
    };

    let create_tables_result = query(
        r#"
        CREATE TABLE IF NOT EXISTS todos (
            id serial PRIMARY KEY,
            title text NOT NULL,
            done boolean NOT NULL DEFAULT false,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
    "#,
    )
    .execute(&pool)
    .await;

    match create_tables_result {
        Ok(r) => {
            tracing::debug!("Create table result {:?}", r);
        }
        Err(e) => {
            tracing::error!("{}", e.to_string());
            panic!("Cannot create tables {DB_STRING}");
        }
    }

    pool
}
