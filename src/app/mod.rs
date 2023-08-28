use prelude::*;

use axum::{response::Redirect, routing::*};

use std::net::SocketAddr;
use tower_http::services::ServeDir;

pub async fn start() {
    let public_dir = ServeDir::new("public");
    let router = Router::new()
        .route("/", get(Redirect::permanent("/todo")))
        .nest("/todo", crate::todo::new().await)
        .nest_service("/public", public_dir.clone())
        .fallback_service(public_dir);
    // run our app with hyper, listening globally on port 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
