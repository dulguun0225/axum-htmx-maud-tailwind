use sqlx::{prelude::*, Pool, Postgres};

#[derive(Debug, FromRow, Clone)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub done: bool,
    // pub created_at: i64,
    // pub updated_at: i64,
}

pub async fn get_all(pool: &Pool<Postgres>) -> Vec<Todo> {
    let todos = sqlx::query_as::<_, Todo>("SELECT id, title, done FROM todos")
        .fetch_all(pool)
        .await
        .unwrap();

    todos
}

pub async fn insert(pool: &Pool<Postgres>, title: &str) -> anyhow::Result<Todo> {
    let todo = sqlx::query_as::<_, Todo>(
        "INSERT INTO todos (title) VALUES ($1) RETURNING id, title, done",
    )
    .bind(title)
    .fetch_one(pool)
    .await?;

    Ok(todo)
}

pub async fn toggle(pool: &Pool<Postgres>, id: &i64) -> anyhow::Result<bool> {
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
