mod app;
mod db;
pub mod prelude;
mod tracing;
pub mod view;

#[tokio::main]
async fn main() {
    tracing::init();
    app::start().await;
}
