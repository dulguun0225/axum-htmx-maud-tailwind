mod app;
mod tracing;
#[tokio::main]
async fn main() {
    tracing::init();
    app::start().await;
}
