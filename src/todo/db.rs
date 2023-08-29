use prelude::*;
use sqlx::prelude::*;

#[derive(Debug, FromRow, Clone)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub done: bool,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
}

pub async fn get_all() -> anyhow::Result<Vec<Todo>> {
    Ok(sqlx::query_as::<_, Todo>(
        r#"
            SELECT *
            FROM todos 
            ORDER BY id"#,
    )
    .fetch_all(pool::get().await)
    .await?)
}

pub async fn insert(title: &str) -> anyhow::Result<Todo> {
    let todo = sqlx::query_as::<_, Todo>("INSERT INTO todos (title) VALUES ($1) RETURNING *")
        .bind(title)
        .fetch_one(pool::get().await)
        .await?;

    Ok(todo)
}

pub async fn toggle(id: &i64) -> anyhow::Result<Todo> {
    let todo = sqlx::query_as::<_, Todo>(
        r#"
        UPDATE todos
        SET 
            done = NOT done,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $1
        RETURNING *"#,
    )
    .bind(id)
    .fetch_one(pool::get().await)
    .await?;

    Ok(todo)
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
