use axum::{
    extract::Path,
    response::IntoResponse,
    routing::*,
    Form, Router,
};
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

async fn insert_handler(Form(data): Form<InsertInput>) -> Markup {
    let todo = db::todo::insert(&data.title).await.unwrap();
    view::todo_list_item(&todo)
}

async fn toggle_handler(Path(id): Path<i64>) -> &'static str {
    let toggle_result = crate::db::todo::toggle(&id).await.unwrap();

    crate::view::todo_done_indicator(toggle_result)
}
async fn delete_handler(Path(id): Path<i64>) {
    db::todo::delete(&id).await.unwrap();
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
