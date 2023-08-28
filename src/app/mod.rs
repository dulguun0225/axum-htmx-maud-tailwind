use axum::{extract::Path, response::IntoResponse, routing::*, Form, Router};
use prelude::*;

use maud::Markup;
use serde::Deserialize;
use std::net::SocketAddr;

use crate::{db, view};

async fn index() -> impl IntoResponse {
    let todos = crate::db::todo::get_all().await;
    crate::view::index(&todos)
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct InsertInput {
    title: String,
}

async fn insert_handler(Form(data): Form<InsertInput>) -> Result<Markup, AppError> {
    let todo = db::todo::insert(&data.title).await?;
    Ok(view::todo_list_item(&todo))
}

async fn toggle_handler(Path(id): Path<i64>) -> Result<&'static str, AppError> {
    let toggle_result = crate::db::todo::toggle(&id).await?;
    Ok(crate::view::todo_done_indicator(toggle_result))
}
async fn delete_handler(Path(id): Path<i64>) -> Result<(), AppError> {
    db::todo::delete(&id).await?;
    Ok(())
}
pub async fn start() {
    db::init().await;
    let router = Router::new()
        .route("/", get(index))
        .route("/", post(insert_handler))
        .route("/:id", patch(toggle_handler))
        .route("/:id", delete(delete_handler));
    // run our app with hyper, listening globally on port 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
