mod app;
mod todo;
mod tracing;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing::init();
    app::start().await?;
    Ok(())
}
