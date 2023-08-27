use axum::{
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use maud::{html, Markup};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tracing::*;
async fn clicked() -> Markup {
    debug!("clicked");
    html! {
        div { "click works" }
    }
}

async fn index() -> impl IntoResponse {
    crate::view::index()
}

pub async fn start() {
    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect("postgrse://postgres:postgres@localhost")
        .await
        .unwrap();

    let router = Router::new()
        .route("/", get(index))
        .route("/", post(clicked));
    // run our app with hyper, listening globally on port 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
