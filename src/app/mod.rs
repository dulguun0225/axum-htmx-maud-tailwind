use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use maud::{html, Markup};
use sqlx::{Pool, Postgres};
use std::{net::SocketAddr, sync::Arc};
use tracing::*;

async fn clicked() -> Markup {
    debug!("clicked");
    html! {
        div { "click works" }
    }
}

async fn index(State(pool): State<Arc<Pool<Postgres>>>) -> impl IntoResponse {
    let todos = crate::db::todo::get_todos(&pool).await;
    debug!("{:?}", todos);
    crate::view::index()
}

pub async fn start() {
    let pool = crate::db::init().await;

    let router = Router::new()
        .route("/", get(index))
        .route("/", post(clicked))
        .with_state(Arc::new(pool));
    // run our app with hyper, listening globally on port 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
