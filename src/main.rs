mod app;
mod db;
mod tracing;
pub mod view;

#[tokio::main]
async fn main() {
    tracing::init();
    app::start().await;
}
