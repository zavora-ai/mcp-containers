use anyhow::{Result, bail};
use reqwest::Client;
use serde_json::Value;

#[derive(Clone)]
pub struct ApiClient {
    pub http: Client,
    pub base_url: String,
    pub auth_header: String,
}

impl ApiClient {
    pub async fn get(&self, path: &str) -> Result<Value> {
        let resp = self.http.get(format!("{}{}", self.base_url, path))
            .header("Authorization", &self.auth_header)
            .send().await?;
        if !resp.status().is_success() { bail!("API {}: {}", resp.status(), resp.text().await?); }
        Ok(resp.json().await?)
    }
    pub async fn post(&self, path: &str, body: &Value) -> Result<Value> {
        let resp = self.http.post(format!("{}{}", self.base_url, path))
            .header("Authorization", &self.auth_header)
            .json(body).send().await?;
        if !resp.status().is_success() { bail!("API {}: {}", resp.status(), resp.text().await?); }
        Ok(resp.json().await?)
    }
    pub async fn delete(&self, path: &str) -> Result<Value> {
        let resp = self.http.delete(format!("{}{}", self.base_url, path))
            .header("Authorization", &self.auth_header)
            .send().await?;
        if !resp.status().is_success() { bail!("API {}: {}", resp.status(), resp.text().await?); }
        Ok(resp.json().await?)
    }
}

/// Container backend — Docker Engine, Kubernetes, or Custom API
#[derive(Clone)]
pub struct ContainerBackend {
    pub runtime: ApiClient,
    pub registry: Option<ApiClient>,
    pub provider: String,
}

impl ContainerBackend {
    pub fn from_env() -> Result<Self> {
        // Docker Engine API (local or remote)
        if let Ok(host) = std::env::var("DOCKER_HOST") {
            let url = if host.starts_with("unix://") { "http://localhost:2375".into() } else { host };
            tracing::info!("Container runtime: Docker Engine");
            let runtime = ApiClient { http: Client::new(), base_url: url, auth_header: String::new() };
            let registry = detect_registry();
            return Ok(Self { runtime, registry, provider: "docker".into() });
        }
        // Kubernetes
        if let Ok(url) = std::env::var("KUBERNETES_API_URL") {
            let token = std::env::var("KUBERNETES_TOKEN").unwrap_or_default();
            tracing::info!("Container runtime: Kubernetes");
            let runtime = ApiClient { http: Client::new(), base_url: url.trim_end_matches('/').into(), auth_header: format!("Bearer {}", token) };
            let registry = detect_registry();
            return Ok(Self { runtime, registry, provider: "kubernetes".into() });
        }
        // Custom API
        if let Ok(url) = std::env::var("CONTAINERS_API_URL") {
            let key = std::env::var("CONTAINERS_API_KEY").unwrap_or_default();
            tracing::info!("Container backend: Custom API");
            let runtime = ApiClient { http: Client::new(), base_url: url.trim_end_matches('/').into(), auth_header: format!("Bearer {}", key) };
            return Ok(Self { runtime, registry: None, provider: "custom".into() });
        }
        bail!("No container backend. Set DOCKER_HOST, KUBERNETES_API_URL, or CONTAINERS_API_URL")
    }
}

fn detect_registry() -> Option<ApiClient> {
    // Docker Hub
    if let Ok(token) = std::env::var("DOCKERHUB_TOKEN") {
        return Some(ApiClient { http: Client::new(), base_url: "https://hub.docker.com/v2".into(), auth_header: format!("Bearer {}", token) });
    }
    // GitHub Container Registry
    if let Ok(token) = std::env::var("GHCR_TOKEN") {
        return Some(ApiClient { http: Client::new(), base_url: "https://ghcr.io/v2".into(), auth_header: format!("Bearer {}", token) });
    }
    // AWS ECR
    if let Ok(token) = std::env::var("ECR_TOKEN") {
        let registry = std::env::var("ECR_REGISTRY").unwrap_or_default();
        return Some(ApiClient { http: Client::new(), base_url: format!("https://{}", registry), auth_header: format!("Basic {}", token) });
    }
    None
}
