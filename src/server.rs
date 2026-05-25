use crate::client::ContainerBackend;
use rmcp::{handler::server::wrapper::Parameters, schemars, tool, tool_router};
use serde_json::json;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct EmptyInput {}
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct IdInput { pub id: String }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct FilterInput { pub status: Option<String>, pub label: Option<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct RunInput { pub image: String, pub name: Option<String>, pub ports: Option<Vec<String>>, pub env: Option<serde_json::Value>, pub command: Option<Vec<String>> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ExecInput { pub container_id: String, pub command: Vec<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct LogsInput { pub container_id: String, pub lines: Option<u32>, pub since: Option<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ImageInput { pub image: String, pub tag: Option<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct BuildInput { pub context: String, pub dockerfile: Option<String>, pub tag: String, pub build_args: Option<serde_json::Value> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct NamespaceInput { pub namespace: Option<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct PodInput { pub name: String, pub namespace: Option<String> }

#[derive(Clone)]
pub struct ContainerServer { pub backend: ContainerBackend }

fn r(result: Result<serde_json::Value, anyhow::Error>) -> String {
    match result { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {}", e) }
}

#[tool_router(server_handler)]
impl ContainerServer {
    // === Containers (7) ===

    #[tool(description = "List running containers (filter by status or label)")]
    async fn list_containers(&self, Parameters(input): Parameters<FilterInput>) -> String {
        let mut path = "/containers?".to_string();
        if let Some(s) = &input.status { path.push_str(&format!("status={}&", s)); }
        if let Some(l) = &input.label { path.push_str(&format!("label={}&", l)); }
        r(self.backend.runtime.get(&path).await)
    }

    #[tool(description = "Get container details: status, ports, mounts, resource usage")]
    async fn get_container(&self, Parameters(input): Parameters<IdInput>) -> String {
        r(self.backend.runtime.get(&format!("/containers/{}", input.id)).await)
    }

    #[tool(description = "Run a new container from an image")]
    async fn run_container(&self, Parameters(input): Parameters<RunInput>) -> String {
        r(self.backend.runtime.post("/containers/run", &json!({
            "image": input.image, "name": input.name, "ports": input.ports,
            "env": input.env, "command": input.command
        })).await)
    }

    #[tool(description = "Stop a running container")]
    async fn stop_container(&self, Parameters(input): Parameters<IdInput>) -> String {
        r(self.backend.runtime.post(&format!("/containers/{}/stop", input.id), &json!({})).await)
    }

    #[tool(description = "Remove a stopped container")]
    async fn remove_container(&self, Parameters(input): Parameters<IdInput>) -> String {
        r(self.backend.runtime.delete(&format!("/containers/{}", input.id)).await)
    }

    #[tool(description = "Get container logs")]
    async fn get_container_logs(&self, Parameters(input): Parameters<LogsInput>) -> String {
        let mut path = format!("/containers/{}/logs?", input.container_id);
        if let Some(n) = input.lines { path.push_str(&format!("lines={}&", n)); }
        if let Some(s) = &input.since { path.push_str(&format!("since={}&", s)); }
        r(self.backend.runtime.get(&path).await)
    }

    #[tool(description = "Execute a command inside a running container")]
    async fn exec_in_container(&self, Parameters(input): Parameters<ExecInput>) -> String {
        r(self.backend.runtime.post(&format!("/containers/{}/exec", input.container_id), &json!({"command": input.command})).await)
    }

    // === Images (5) ===

    #[tool(description = "List local images")]
    async fn list_images(&self, Parameters(_): Parameters<EmptyInput>) -> String {
        r(self.backend.runtime.get("/images").await)
    }

    #[tool(description = "Pull an image from a registry")]
    async fn pull_image(&self, Parameters(input): Parameters<ImageInput>) -> String {
        r(self.backend.runtime.post("/images/pull", &json!({"image": input.image, "tag": input.tag.unwrap_or("latest".into())})).await)
    }

    #[tool(description = "Build an image from a Dockerfile")]
    async fn build_image(&self, Parameters(input): Parameters<BuildInput>) -> String {
        r(self.backend.runtime.post("/images/build", &json!({
            "context": input.context, "dockerfile": input.dockerfile.unwrap_or("Dockerfile".into()),
            "tag": input.tag, "build_args": input.build_args
        })).await)
    }

    #[tool(description = "Push an image to a registry")]
    async fn push_image(&self, Parameters(input): Parameters<ImageInput>) -> String {
        r(self.backend.runtime.post("/images/push", &json!({"image": input.image, "tag": input.tag.unwrap_or("latest".into())})).await)
    }

    #[tool(description = "Remove a local image")]
    async fn remove_image(&self, Parameters(input): Parameters<ImageInput>) -> String {
        r(self.backend.runtime.delete(&format!("/images/{}:{}", input.image, input.tag.unwrap_or("latest".into()))).await)
    }

    // === Registry (4) ===

    #[tool(description = "List repositories in the container registry")]
    async fn list_repositories(&self, Parameters(_): Parameters<EmptyInput>) -> String {
        match &self.backend.registry {
            Some(reg) => r(reg.get("/repositories").await),
            None => "Error: No registry configured".into(),
        }
    }

    #[tool(description = "List tags for a repository")]
    async fn list_tags(&self, Parameters(input): Parameters<ImageInput>) -> String {
        match &self.backend.registry {
            Some(reg) => r(reg.get(&format!("/repositories/{}/tags", input.image)).await),
            None => "Error: No registry configured".into(),
        }
    }

    #[tool(description = "Get image manifest and layers")]
    async fn get_manifest(&self, Parameters(input): Parameters<ImageInput>) -> String {
        match &self.backend.registry {
            Some(reg) => r(reg.get(&format!("/repositories/{}/manifests/{}", input.image, input.tag.unwrap_or("latest".into()))).await),
            None => "Error: No registry configured".into(),
        }
    }

    #[tool(description = "Scan image for vulnerabilities")]
    async fn scan_image(&self, Parameters(input): Parameters<ImageInput>) -> String {
        match &self.backend.registry {
            Some(reg) => r(reg.get(&format!("/repositories/{}/scan?tag={}", input.image, input.tag.unwrap_or("latest".into()))).await),
            None => r(self.backend.runtime.post("/images/scan", &json!({"image": input.image, "tag": input.tag})).await),
        }
    }

    // === Kubernetes Pods (4) ===

    #[tool(description = "List pods in a namespace")]
    async fn list_pods(&self, Parameters(input): Parameters<NamespaceInput>) -> String {
        let ns = input.namespace.unwrap_or("default".into());
        r(self.backend.runtime.get(&format!("/pods?namespace={}", ns)).await)
    }

    #[tool(description = "Get pod details: status, containers, events")]
    async fn get_pod(&self, Parameters(input): Parameters<PodInput>) -> String {
        let ns = input.namespace.unwrap_or("default".into());
        r(self.backend.runtime.get(&format!("/pods/{}/{}",  ns, input.name)).await)
    }

    #[tool(description = "Delete a pod (triggers reschedule)")]
    async fn delete_pod(&self, Parameters(input): Parameters<PodInput>) -> String {
        let ns = input.namespace.unwrap_or("default".into());
        r(self.backend.runtime.delete(&format!("/pods/{}/{}", ns, input.name)).await)
    }

    #[tool(description = "Get pod logs")]
    async fn get_pod_logs(&self, Parameters(input): Parameters<PodInput>) -> String {
        let ns = input.namespace.unwrap_or("default".into());
        r(self.backend.runtime.get(&format!("/pods/{}/{}/logs", ns, input.name)).await)
    }

    // === System (2) ===

    #[tool(description = "Get system info: Docker version, OS, resources, storage")]
    async fn get_system_info(&self, Parameters(_): Parameters<EmptyInput>) -> String {
        r(self.backend.runtime.get("/system/info").await)
    }

    #[tool(description = "Prune unused containers, images, and volumes")]
    async fn prune(&self, Parameters(_): Parameters<EmptyInput>) -> String {
        r(self.backend.runtime.post("/system/prune", &json!({})).await)
    }
}
