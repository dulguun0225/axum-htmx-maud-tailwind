use prelude::*;
use sqlx::prelude::*;

pub async fn init() {
    let create_tables_result = sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS todos (
            id serial PRIMARY KEY,
            title text NOT NULL,
            CHECK (title <> ''),
            done boolean NOT NULL DEFAULT false,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
    "#,
    )
    .execute(pool::get().await)
    .await;

    match create_tables_result {
        Ok(r) => {
            debug!("Create table result {:?}", r);
        }
        Err(e) => {
            tracing::error!("{}", e.to_string());
            panic!("Cannot create tables");
        }
    }
}
#[derive(Debug, FromRow, Clone)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub done: bool,
    // pub created_at: i64,
    // pub updated_at: i64,
}

pub async fn get_all() -> Vec<Todo> {
    sqlx::query_as::<_, Todo>(
        r#"
            SELECT id, title, done 
            FROM todos 
            ORDER BY id"#,
    )
    .fetch_all(pool::get().await)
    .await
    .unwrap()
}

pub async fn insert(title: &str) -> anyhow::Result<Todo> {
    let todo = sqlx::query_as::<_, Todo>("INSERT INTO todos (title) VALUES ($1) RETURNING *")
        .bind(title)
        .fetch_one(pool::get().await)
        .await?;

    Ok(todo)
}

pub async fn toggle(id: &i64) -> anyhow::Result<bool> {
    let res: (bool,) = sqlx::query_as(
        r#"
        UPDATE todos
        SET done = NOT done
        WHERE id = $1
        RETURNING done"#,
    )
    .bind(id)
    .fetch_one(pool::get().await)
    .await?;

    Ok(res.0)
}

pub async fn delete(id: &i64) -> anyhow::Result<Todo> {
    let todo = sqlx::query_as::<_, Todo>(
        r#"
        DELETE FROM todos
        WHERE id = $1
        RETURNING *
    "#,
    )
    .bind(id)
    .fetch_one(pool::get().await)
    .await?;

    Ok(todo)
}
