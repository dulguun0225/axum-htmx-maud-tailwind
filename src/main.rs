mod app;
mod todo;
mod tracing;

#[tokio::main]
async fn main() {
    tracing::init();
    app::start().await;
}
