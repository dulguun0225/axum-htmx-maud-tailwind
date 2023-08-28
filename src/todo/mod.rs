mod db;
mod view;

use axum::{extract::Path, response::IntoResponse, routing::*, Form, Router};
use prelude::*;

use maud::Markup;
use serde::Deserialize;

async fn index() -> impl IntoResponse {
    let todos = db::get_all().await;
    view::index(&todos)
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct InsertInput {
    title: String,
}

async fn insert_handler(Form(data): Form<InsertInput>) -> Result<Markup, AppError> {
    let todo = db::insert(&data.title).await?;
    Ok(view::todo_list_item(&todo))
}

async fn toggle_handler(Path(id): Path<i64>) -> Result<&'static str, AppError> {
    let toggle_result = db::toggle(&id).await?;
    Ok(view::todo_done_indicator(toggle_result))
}
async fn delete_handler(Path(id): Path<i64>) -> Result<(), AppError> {
    db::delete(&id).await?;
    Ok(())
}
pub async fn new() -> Router {
    db::init().await;
    Router::new()
        .route("/", get(index).post(insert_handler))
        .route("/:id", patch(toggle_handler).delete(delete_handler))
}
