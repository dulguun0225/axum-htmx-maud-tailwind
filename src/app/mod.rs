use axum::{
    extract::{RawForm, State, Path},
    response::IntoResponse,
    routing::{get, post},
    Form, Router,
};
use maud::{html, Markup};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::net::SocketAddr;
use tracing::*;

use crate::{db::todo::insert, view::todo_list_item};

async fn clicked() -> Markup {
    debug!("clicked");
    html! {
        div { "click works" }
    }
}

async fn index(State(pool): State<Pool<Postgres>>) -> impl IntoResponse {
    let todos = crate::db::todo::get_all(&pool).await;
    crate::view::index(&todos)
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct InsertInput {
    title: String,
}

async fn insert_handler(State(pool): State<Pool<Postgres>>, Form(data): Form<InsertInput>) -> Markup {
    let todo = insert(&pool, &data.title).await.unwrap();
    todo_list_item(&todo)
}

async fn toggle_handler(Path(id): Path<i64>, State(pool): State<Pool<Postgres>>) -> &'static str {
    let toggle_result = crate::db::todo::toggle(&pool, &id).await.unwrap();

    crate::view::todo_done_indicator(toggle_result)
}

pub async fn start() {
    let pool = crate::db::init().await;

    let router = Router::new()
        .route("/", get(index))
        .route("/", post(clicked))
        .route("/insert", post(insert_handler))
        .route("/toggle/:id", post(toggle_handler))
        .with_state(pool);
    // run our app with hyper, listening globally on port 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
