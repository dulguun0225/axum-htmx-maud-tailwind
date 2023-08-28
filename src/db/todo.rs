use crate::db::get_pool;
use sqlx::prelude::*;
#[derive(Debug, FromRow, Clone)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub done: bool,
    // pub created_at: i64,
    // pub updated_at: i64,
}

pub async fn get_all() -> Vec<Todo> {
    let pool = get_pool().await;
    sqlx::query_as::<_, Todo>(
        r#"
            SELECT id, title, done 
            FROM todos 
            ORDER BY id"#,
    )
    .fetch_all(pool)
    .await
    .unwrap()
}

pub async fn insert(title: &str) -> anyhow::Result<Todo> {
    let pool = get_pool().await;
    let todo = sqlx::query_as::<_, Todo>(
        "INSERT INTO todos (title) VALUES ($1) RETURNING id, title, done",
    )
    .bind(title)
    .fetch_one(pool)
    .await?;

    Ok(todo)
}

pub async fn toggle(id: &i64) -> anyhow::Result<bool> {
    let pool = get_pool().await;
    let res: (bool,) = sqlx::query_as(
        r#"
        UPDATE todos
        SET done = NOT done
        WHERE id = $1
        RETURNING done"#,
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(res.0)
}

pub async fn delete(id: &i64) -> anyhow::Result<()> {
    let pool = get_pool().await;
    let res = sqlx::query(
        r#"
        DELETE FROM todos
        WHERE id = $1
    "#,
    )
    .bind(id)
    .execute(pool)
    .await?;

    tracing::debug!("{:?}", res);

    Ok(())
}
