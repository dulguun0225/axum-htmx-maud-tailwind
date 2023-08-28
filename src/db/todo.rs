use crate::db::get_pool;
use sqlx::prelude::*;
// use tracing::*;

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
    .fetch_all(get_pool().await)
    .await
    .unwrap()
}

pub async fn insert(title: &str) -> anyhow::Result<Todo> {
    let todo = sqlx::query_as::<_, Todo>("INSERT INTO todos (title) VALUES ($1) RETURNING *")
        .bind(title)
        .fetch_one(get_pool().await)
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
    .fetch_one(get_pool().await)
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
    .fetch_one(get_pool().await)
    .await?;

    Ok(todo)
}
