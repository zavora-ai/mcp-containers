mod server;

use bollard::Docker;
use rmcp::{ServiceExt, transport::stdio};
use server::ContainerServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_env_filter(tracing_subscriber::EnvFilter::from_default_env()).init();
    let docker = Docker::connect_with_local_defaults()?;
    let version = docker.version().await?;
    tracing::info!("Connected to Docker {}", version.version.as_deref().unwrap_or("unknown"));
    let service = ContainerServer { docker }.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
