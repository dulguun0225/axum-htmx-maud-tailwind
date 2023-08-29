mod todo;
mod tracing;
mod web;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing::init();
    web::start().await?;
    Ok(())
}
