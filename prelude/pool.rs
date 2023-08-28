use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::sync::OnceCell;
use tracing::*;

pub static DB_STRING: &str = "postgres://postgres:postgres@localhost/todos";

static POOL: OnceCell<Pool<Postgres>> = OnceCell::const_new();

pub async fn get() -> &'static Pool<Postgres> {
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

pub async fn init() {}
