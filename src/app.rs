use axum::{response::IntoResponse, routing::get, Router};
use maud::{html, Markup};
use std::net::SocketAddr;

async fn index() -> impl IntoResponse {
    html! {
        h1 { "Hello World"}
    }
}

pub async fn start() {
    let router = Router::new().route("/", get(index));
    // run our app with hyper, listening globally on port 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
