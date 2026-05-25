mod client;
mod server;

use client::ContainerBackend;
use rmcp::{ServiceExt, transport::stdio};
use server::ContainerServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_env_filter(tracing_subscriber::EnvFilter::from_default_env()).init();
    let backend = ContainerBackend::from_env()?;
    let service = ContainerServer { backend }.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
