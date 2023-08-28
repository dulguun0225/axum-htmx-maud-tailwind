pub mod todo;

use sqlx::{postgres::PgPoolOptions, query, Pool, Postgres};
use tokio::sync::OnceCell;
use tracing::*;

pub static DB_STRING: &str = "postgres://postgres:postgres@localhost/todos";

static POOL: OnceCell<Pool<Postgres>> = OnceCell::const_new();

pub async fn get_pool() -> &'static Pool<Postgres> {
    POOL.get_or_init(|| async {
        let result = PgPoolOptions::new()
            .max_connections(8)
            .connect(DB_STRING)
            .await;

        match result {
            Ok(new_pool) => new_pool,
            Err(e) => {
                error!("DB Error. Cannot connect to {}. {}", DB_STRING, e);
                panic!("{e:?}");
            }
        }
    })
    .await
}

pub async fn init() {
    let pool = get_pool().await;
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
    .execute(pool)
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
}
