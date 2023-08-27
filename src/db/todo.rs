use sqlx::{prelude::*, Pool, Postgres};

// use std::time;

#[derive(Debug, FromRow, Clone)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub done: bool,
    pub created_at: i64,
    pub updated_at: i64,
}


pub async fn get_todos(pool: &Pool<Postgres>) -> Vec<Todo> {
    let todos = sqlx::query_as::<_, Todo>("SELECT id, title, done FROM todos")
        .fetch_all(pool)
        .await
        .unwrap();

    todos
}
