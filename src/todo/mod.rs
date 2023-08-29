mod db;
mod view;

use axum::{extract::Path, routing::*, Form, Router};
use prelude::*;

use maud::Markup;
use serde::Deserialize;

async fn index() -> Result<Markup, AppError> {
    let todos = db::get_all().await?;
    info!("{todos:?}");
    Ok(view::index(&todos))
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

async fn toggle_handler(Path(id): Path<i64>) -> Result<Markup, AppError> {
    let toggle_result = db::toggle(&id).await?;
    Ok(view::todo_list_item(&toggle_result))
}
async fn delete_handler(Path(id): Path<i64>) -> Result<(), AppError> {
    db::delete(&id).await?;
    Ok(())
}
pub async fn new() -> Router {
    Router::new()
        .route("/", get(index).post(insert_handler))
        .route("/:id", patch(toggle_handler).delete(delete_handler))
}
