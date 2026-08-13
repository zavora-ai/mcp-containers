use bollard::container::*;
use bollard::exec::*;
use bollard::image::*;
use bollard::network::*;
use bollard::volume::*;
use bollard::Docker;
use bollard::models::*;
use futures_util::StreamExt;
use rmcp::{handler::server::wrapper::Parameters, schemars, tool, tool_router};
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct EmptyInput {}
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct IdInput { pub id: String }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct RunInput { pub image: String, pub name: Option<String>, pub ports: Option<Vec<String>>, pub env: Option<Vec<String>>, pub cmd: Option<Vec<String>> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ExecInput { pub container_id: String, pub cmd: Vec<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct LogsInput { pub container_id: String, pub tail: Option<u32> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ImageInput { pub image: String, pub tag: Option<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct FilterInput { pub all: Option<bool> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct NameInput { pub name: String }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct NetworkInput { pub name: String, pub driver: Option<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct VolumeInput { pub name: String }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct RenameInput { pub id: String, pub new_name: String }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct TagInput { pub image: String, pub repo: String, pub tag: String }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ConnectInput { pub network: String, pub container_id: String }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ComposeInput { pub path: Option<String>, pub file: Option<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CopyInput { pub container_id: String, pub container_path: String, pub local_path: String }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct UpdateContainerInput { pub id: String, pub memory_bytes: Option<i64>, pub cpus: Option<f64> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ExportInput { pub id: String, pub output: Option<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct LoadInput { pub path: String }

#[derive(Clone)]
pub struct ContainerServer {
    pub docker: Docker,
}

#[tool_router]
impl ContainerServer {
    // === Containers (8) ===

    #[tool(description = "List containers (set all=true to include stopped)")]
    async fn list_containers(&self, Parameters(input): Parameters<FilterInput>) -> String {
        let opts = ListContainersOptions::<String> { all: input.all.unwrap_or(false), ..Default::default() };
        match self.docker.list_containers(Some(opts)).await {
            Ok(containers) => {
                let summary: Vec<serde_json::Value> = containers.iter().map(|c| json!({
                    "id": c.id.as_deref().unwrap_or("").get(..12).unwrap_or(""),
                    "names": c.names, "image": c.image, "state": c.state,
                    "status": c.status, "ports": c.ports.as_ref().map(|ps| ps.iter().map(|p| format!("{}:{}", p.public_port.unwrap_or(0), p.private_port)).collect::<Vec<_>>())
                })).collect();
                serde_json::to_string_pretty(&summary).unwrap()
            }
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Get container details: status, config, mounts, network")]
    async fn inspect_container(&self, Parameters(input): Parameters<IdInput>) -> String {
        match self.docker.inspect_container(&input.id, None).await {
            Ok(info) => {
                let state = info.state.as_ref();
                serde_json::to_string_pretty(&json!({
                    "id": info.id.as_deref().unwrap_or("").get(..12).unwrap_or(""),
                    "name": info.name, "image": info.config.as_ref().and_then(|c| c.image.as_ref()),
                    "status": state.and_then(|s| s.status.as_ref()),
                    "started_at": state.and_then(|s| s.started_at.as_ref()),
                    "ports": info.network_settings.as_ref().and_then(|n| n.ports.as_ref()),
                    "env": info.config.as_ref().and_then(|c| c.env.as_ref()),
                    "mounts": info.mounts,
                })).unwrap()
            }
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Run a new container from an image")]
    async fn run_container(&self, Parameters(input): Parameters<RunInput>) -> String {
        let mut port_bindings = HashMap::new();
        if let Some(ports) = &input.ports {
            for p in ports {
                let parts: Vec<&str> = p.split(':').collect();
                if parts.len() == 2 {
                    port_bindings.insert(format!("{}/tcp", parts[1]), Some(vec![PortBinding { host_ip: Some("0.0.0.0".into()), host_port: Some(parts[0].into()) }]));
                }
            }
        }
        let config = Config {
            image: Some(input.image.clone()),
            env: input.env.clone(),
            cmd: input.cmd.clone(),
            host_config: Some(HostConfig { port_bindings: Some(port_bindings), ..Default::default() }),
            ..Default::default()
        };
        let name = input.name.as_deref().unwrap_or("");
        let opts = if name.is_empty() { None } else { Some(CreateContainerOptions { name, platform: None }) };
        match self.docker.create_container(opts, config).await {
            Ok(resp) => {
                let _ = self.docker.start_container::<String>(&resp.id, None).await;
                format!("Container started: {} ({})", input.name.unwrap_or(input.image), &resp.id[..12])
            }
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Stop a running container")]
    async fn stop_container(&self, Parameters(input): Parameters<IdInput>) -> String {
        match self.docker.stop_container(&input.id, None).await {
            Ok(_) => format!("Container {} stopped", input.id),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Remove a container (must be stopped first)")]
    async fn remove_container(&self, Parameters(input): Parameters<IdInput>) -> String {
        match self.docker.remove_container(&input.id, None).await {
            Ok(_) => format!("Container {} removed", input.id),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Get container logs (last N lines)")]
    async fn get_logs(&self, Parameters(input): Parameters<LogsInput>) -> String {
        let opts = LogsOptions::<String> { stdout: true, stderr: true, tail: input.tail.unwrap_or(50).to_string(), ..Default::default() };
        let mut stream = self.docker.logs(&input.container_id, Some(opts));
        let mut lines = Vec::new();
        while let Some(Ok(log)) = stream.next().await {
            lines.push(log.to_string());
            if lines.len() >= 100 { break; }
        }
        lines.join("")
    }

    #[tool(description = "Execute a command inside a running container")]
    async fn exec_container(&self, Parameters(input): Parameters<ExecInput>) -> String {
        let config = CreateExecOptions { cmd: Some(input.cmd.clone()), attach_stdout: Some(true), attach_stderr: Some(true), ..Default::default() };
        match self.docker.create_exec(&input.container_id, config).await {
            Ok(exec) => {
                match self.docker.start_exec(&exec.id, None).await {
                    Ok(StartExecResults::Attached { mut output, .. }) => {
                        let mut result = String::new();
                        while let Some(Ok(msg)) = output.next().await {
                            result.push_str(&msg.to_string());
                        }
                        result
                    }
                    Ok(StartExecResults::Detached) => "Executed (detached)".into(),
                    Err(e) => format!("Error: {}", e),
                }
            }
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Get container resource usage (CPU, memory, network I/O)")]
    async fn get_stats(&self, Parameters(input): Parameters<IdInput>) -> String {
        let opts = StatsOptions { stream: false, one_shot: true };
        let mut stream = self.docker.stats(&input.id, Some(opts));
        if let Some(Ok(stats)) = stream.next().await {
            let mem = stats.memory_stats.usage.unwrap_or(0);
            let mem_limit = stats.memory_stats.limit.unwrap_or(1);
            let cpu = stats.cpu_stats.cpu_usage.total_usage;
            serde_json::to_string_pretty(&json!({
                "memory_mb": mem / 1024 / 1024,
                "memory_limit_mb": mem_limit / 1024 / 1024,
                "memory_pct": (mem as f64 / mem_limit as f64 * 100.0).round(),
                "cpu_total": cpu,
                "network_rx_mb": stats.networks.as_ref().map(|n| n.values().map(|v| v.rx_bytes).sum::<u64>() / 1024 / 1024),
                "network_tx_mb": stats.networks.as_ref().map(|n| n.values().map(|v| v.tx_bytes).sum::<u64>() / 1024 / 1024),
            })).unwrap()
        } else {
            "Error: could not get stats".into()
        }
    }

    // === Images (4) ===

    #[tool(description = "List local Docker images")]
    async fn list_images(&self, Parameters(_): Parameters<EmptyInput>) -> String {
        match self.docker.list_images(Some(ListImagesOptions::<String> { all: false, ..Default::default() })).await {
            Ok(images) => {
                let summary: Vec<serde_json::Value> = images.iter().map(|i| json!({
                    "id": i.id.get(7..19).unwrap_or(&i.id),
                    "tags": i.repo_tags, "size_mb": i.size / 1024 / 1024,
                    "created": i.created
                })).collect();
                serde_json::to_string_pretty(&summary).unwrap()
            }
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Pull an image from a registry")]
    async fn pull_image(&self, Parameters(input): Parameters<ImageInput>) -> String {
        let tag = input.tag.as_deref().unwrap_or("latest");
        let opts = CreateImageOptions { from_image: input.image.as_str(), tag, ..Default::default() };
        let mut stream = self.docker.create_image(Some(opts), None, None);
        let mut last_status = String::new();
        while let Some(Ok(info)) = stream.next().await {
            if let Some(s) = info.status { last_status = s; }
        }
        format!("Pulled {}:{} — {}", input.image, tag, last_status)
    }

    #[tool(description = "Remove a local image")]
    async fn remove_image(&self, Parameters(input): Parameters<ImageInput>) -> String {
        let tag = input.tag.as_deref().unwrap_or("latest");
        let full = format!("{}:{}", input.image, tag);
        match self.docker.remove_image(&full, None, None).await {
            Ok(_) => format!("Removed image {}", full),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Inspect an image: layers, size, config, history")]
    async fn inspect_image(&self, Parameters(input): Parameters<ImageInput>) -> String {
        let tag = input.tag.as_deref().unwrap_or("latest");
        let full = format!("{}:{}", input.image, tag);
        match self.docker.inspect_image(&full).await {
            Ok(info) => serde_json::to_string_pretty(&json!({
                "id": info.id.as_deref().unwrap_or("").get(7..19),
                "size_mb": info.size.unwrap_or(0) / 1024 / 1024,
                "os": info.os, "architecture": info.architecture,
                "created": info.created,
                "env": info.config.as_ref().and_then(|c| c.env.as_ref()),
                "cmd": info.config.as_ref().and_then(|c| c.cmd.as_ref()),
            })).unwrap(),
            Err(e) => format!("Error: {}", e),
        }
    }

    // === Networks (2) ===

    #[tool(description = "List Docker networks")]
    async fn list_networks(&self, Parameters(_): Parameters<EmptyInput>) -> String {
        match self.docker.list_networks(None::<ListNetworksOptions<String>>).await {
            Ok(nets) => {
                let summary: Vec<serde_json::Value> = nets.iter().map(|n| json!({
                    "id": n.id.as_deref().unwrap_or("").get(..12).unwrap_or(""),
                    "name": n.name, "driver": n.driver, "scope": n.scope
                })).collect();
                serde_json::to_string_pretty(&summary).unwrap()
            }
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Create a Docker network")]
    async fn create_network(&self, Parameters(input): Parameters<NetworkInput>) -> String {
        let config = CreateNetworkOptions { name: input.name.as_str(), driver: input.driver.as_deref().unwrap_or("bridge"), ..Default::default() };
        match self.docker.create_network(config).await {
            Ok(resp) => format!("Network '{}' created ({})", input.name, resp.id.get(..12).unwrap_or(&resp.id)),
            Err(e) => format!("Error: {}", e),
        }
    }

    // === Volumes (2) ===

    #[tool(description = "List Docker volumes")]
    async fn list_volumes(&self, Parameters(_): Parameters<EmptyInput>) -> String {
        match self.docker.list_volumes(None::<ListVolumesOptions<String>>).await {
            Ok(resp) => {
                let vols: Vec<serde_json::Value> = resp.volumes.unwrap_or_default().iter().map(|v| json!({
                    "name": v.name, "driver": v.driver, "mountpoint": v.mountpoint
                })).collect();
                serde_json::to_string_pretty(&vols).unwrap()
            }
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Create a Docker volume")]
    async fn create_volume(&self, Parameters(input): Parameters<VolumeInput>) -> String {
        let config = CreateVolumeOptions { name: input.name.as_str(), ..Default::default() };
        match self.docker.create_volume(config).await {
            Ok(v) => format!("Volume '{}' created at {}", v.name, v.mountpoint),
            Err(e) => format!("Error: {}", e),
        }
    }

    // === System (2) ===

    #[tool(description = "Get Docker system info: version, OS, containers, images count")]
    async fn system_info(&self, Parameters(_): Parameters<EmptyInput>) -> String {
        match self.docker.info().await {
            Ok(info) => serde_json::to_string_pretty(&json!({
                "containers": info.containers, "running": info.containers_running,
                "stopped": info.containers_stopped, "images": info.images,
                "server_version": info.server_version, "os": info.operating_system,
                "arch": info.architecture, "cpus": info.ncpu,
                "memory_gb": info.mem_total.unwrap_or(0) / 1024 / 1024 / 1024,
            })).unwrap(),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Prune stopped containers, unused images, and dangling volumes")]
    async fn prune(&self, Parameters(_): Parameters<EmptyInput>) -> String {
        let c = self.docker.prune_containers(None::<PruneContainersOptions<String>>).await.ok();
        let i = self.docker.prune_images(None::<PruneImagesOptions<String>>).await.ok();
        let v = self.docker.prune_volumes(None::<PruneVolumesOptions<String>>).await.ok();
        serde_json::to_string_pretty(&json!({
            "containers_removed": c.as_ref().and_then(|r| r.containers_deleted.as_ref()).map(|v| v.len()),
            "images_removed": i.as_ref().and_then(|r| r.images_deleted.as_ref()).map(|v| v.len()),
            "volumes_removed": v.as_ref().and_then(|r| r.volumes_deleted.as_ref()).map(|v| v.len()),
            "space_reclaimed_mb": i.as_ref().map(|r| r.space_reclaimed.unwrap_or(0) / 1024 / 1024).unwrap_or(0),
        })).unwrap()
    }

    // === Additional Container Operations ===

    #[tool(description = "Restart a running container")]
    async fn restart_container(&self, Parameters(input): Parameters<IdInput>) -> String {
        match self.docker.restart_container(&input.id, None).await {
            Ok(_) => format!("Container {} restarted", input.id),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Pause a running container (freeze all processes)")]
    async fn pause_container(&self, Parameters(input): Parameters<IdInput>) -> String {
        match self.docker.pause_container(&input.id).await {
            Ok(_) => format!("Container {} paused", input.id),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Unpause a paused container")]
    async fn unpause_container(&self, Parameters(input): Parameters<IdInput>) -> String {
        match self.docker.unpause_container(&input.id).await {
            Ok(_) => format!("Container {} unpaused", input.id),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Rename a container")]
    async fn rename_container(&self, Parameters(input): Parameters<RenameInput>) -> String {
        match self.docker.rename_container(&input.id, RenameContainerOptions { name: &input.new_name }).await {
            Ok(_) => format!("Container {} renamed to {}", input.id, input.new_name),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Show running processes inside a container")]
    async fn get_top(&self, Parameters(input): Parameters<IdInput>) -> String {
        match self.docker.top_processes(&input.id, None::<TopOptions<String>>).await {
            Ok(top) => serde_json::to_string_pretty(&json!({
                "titles": top.titles, "processes": top.processes
            })).unwrap(),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Wait for a container to exit and return the exit code")]
    async fn wait_container(&self, Parameters(input): Parameters<IdInput>) -> String {
        let mut stream = self.docker.wait_container::<String>(&input.id, None);
        if let Some(Ok(result)) = stream.next().await {
            format!("Container {} exited with code {}", input.id, result.status_code)
        } else {
            format!("Error waiting for container {}", input.id)
        }
    }

    // === Additional Image Operations ===

    #[tool(description = "Tag an image (e.g. myapp:latest → myapp:v2.0)")]
    async fn tag_image(&self, Parameters(input): Parameters<TagInput>) -> String {
        let opts = TagImageOptions { repo: input.repo.as_str(), tag: input.tag.as_str() };
        match self.docker.tag_image(&input.image, Some(opts)).await {
            Ok(_) => format!("Tagged {} as {}:{}", input.image, input.repo, input.tag),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Show image layer history")]
    async fn image_history(&self, Parameters(input): Parameters<ImageInput>) -> String {
        let full = format!("{}:{}", input.image, input.tag.as_deref().unwrap_or("latest"));
        match self.docker.image_history(&full).await {
            Ok(history) => {
                let layers: Vec<serde_json::Value> = history.iter().map(|h| json!({
                    "created_by": h.created_by, "size_mb": h.size / 1024 / 1024,
                })).collect();
                serde_json::to_string_pretty(&layers).unwrap()
            }
            Err(e) => format!("Error: {}", e),
        }
    }

    // === Additional Network Operations ===

    #[tool(description = "Remove a Docker network")]
    async fn remove_network(&self, Parameters(input): Parameters<NameInput>) -> String {
        match self.docker.remove_network(&input.name).await {
            Ok(_) => format!("Network {} removed", input.name),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Connect a container to a network")]
    async fn connect_network(&self, Parameters(input): Parameters<ConnectInput>) -> String {
        let config = ConnectNetworkOptions { container: &input.container_id, endpoint_config: EndpointSettings::default() };
        match self.docker.connect_network(&input.network, config).await {
            Ok(_) => format!("Container {} connected to network {}", input.container_id, input.network),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Disconnect a container from a network")]
    async fn disconnect_network(&self, Parameters(input): Parameters<ConnectInput>) -> String {
        let config = DisconnectNetworkOptions { container: &input.container_id, force: true };
        match self.docker.disconnect_network(&input.network, config).await {
            Ok(_) => format!("Container {} disconnected from network {}", input.container_id, input.network),
            Err(e) => format!("Error: {}", e),
        }
    }

    // === Additional Volume Operations ===

    #[tool(description = "Remove a Docker volume")]
    async fn remove_volume(&self, Parameters(input): Parameters<VolumeInput>) -> String {
        match self.docker.remove_volume(&input.name, None).await {
            Ok(_) => format!("Volume {} removed", input.name),
            Err(e) => format!("Error: {}", e),
        }
    }

    // === Docker Compose ===

    #[tool(description = "Start a docker-compose stack (docker compose up -d)")]
    async fn compose_up(&self, Parameters(input): Parameters<ComposeInput>) -> String {
        let dir = input.path.as_deref().unwrap_or(".");
        let file = input.file.as_deref().unwrap_or("docker-compose.yml");
        let output = tokio::process::Command::new("docker")
            .args(["compose", "-f", file, "up", "-d"])
            .current_dir(dir)
            .output().await;
        match output {
            Ok(o) if o.status.success() => format!("Compose stack started in {}", dir),
            Ok(o) => format!("Error: {}", String::from_utf8_lossy(&o.stderr)),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Stop a docker-compose stack (docker compose down)")]
    async fn compose_down(&self, Parameters(input): Parameters<ComposeInput>) -> String {
        let dir = input.path.as_deref().unwrap_or(".");
        let file = input.file.as_deref().unwrap_or("docker-compose.yml");
        let output = tokio::process::Command::new("docker")
            .args(["compose", "-f", file, "down"])
            .current_dir(dir)
            .output().await;
        match output {
            Ok(o) if o.status.success() => format!("Compose stack stopped in {}", dir),
            Ok(o) => format!("Error: {}", String::from_utf8_lossy(&o.stderr)),
            Err(e) => format!("Error: {}", e),
        }
    }

    // === File Copy ===

    #[tool(description = "Copy a file from a container to local path")]
    async fn copy_from_container(&self, Parameters(input): Parameters<CopyInput>) -> String {
        let output = tokio::process::Command::new("docker")
            .args(["cp", &format!("{}:{}", input.container_id, input.container_path), &input.local_path])
            .output().await;
        match output {
            Ok(o) if o.status.success() => format!("Copied {}:{} → {}", input.container_id, input.container_path, input.local_path),
            Ok(o) => format!("Error: {}", String::from_utf8_lossy(&o.stderr)),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Copy a file from local path into a container")]
    async fn copy_to_container(&self, Parameters(input): Parameters<CopyInput>) -> String {
        let output = tokio::process::Command::new("docker")
            .args(["cp", &input.local_path, &format!("{}:{}", input.container_id, input.container_path)])
            .output().await;
        match output {
            Ok(o) if o.status.success() => format!("Copied {} → {}:{}", input.local_path, input.container_id, input.container_path),
            Ok(o) => format!("Error: {}", String::from_utf8_lossy(&o.stderr)),
            Err(e) => format!("Error: {}", e),
        }
    }

    // === Additional Operations ===

    #[tool(description = "Kill a container (send SIGKILL or custom signal)")]
    async fn kill_container(&self, Parameters(input): Parameters<IdInput>) -> String {
        match self.docker.kill_container::<String>(&input.id, None).await {
            Ok(_) => format!("Container {} killed", input.id),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Get filesystem changes in a container (added/modified/deleted files)")]
    async fn get_changes(&self, Parameters(input): Parameters<IdInput>) -> String {
        match self.docker.container_changes(&input.id).await {
            Ok(Some(changes)) => {
                let items: Vec<serde_json::Value> = changes.iter().map(|c| json!({
                    "path": c.path, "kind": c.kind
                })).collect();
                serde_json::to_string_pretty(&items).unwrap()
            }
            Ok(None) => "No changes".into(),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Update container resource limits (CPU, memory)")]
    async fn update_container(&self, Parameters(input): Parameters<UpdateContainerInput>) -> String {
        let config = UpdateContainerOptions::<String> {
            memory: input.memory_bytes,
            nano_cpus: input.cpus.map(|c| (c * 1e9) as i64),
            ..Default::default()
        };
        match self.docker.update_container(&input.id, config).await {
            Ok(_) => format!("Container {} resources updated", input.id),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Export container filesystem as tar (returns file path)")]
    async fn export_container(&self, Parameters(input): Parameters<ExportInput>) -> String {
        let output_path = input.output.unwrap_or(format!("{}.tar", input.id));
        let output = tokio::process::Command::new("docker")
            .args(["export", "-o", &output_path, &input.id])
            .output().await;
        match output {
            Ok(o) if o.status.success() => format!("Exported container {} → {}", input.id, output_path),
            Ok(o) => format!("Error: {}", String::from_utf8_lossy(&o.stderr)),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Inspect a Docker network (details, connected containers)")]
    async fn inspect_network(&self, Parameters(input): Parameters<NameInput>) -> String {
        match self.docker.inspect_network::<String>(&input.name, None).await {
            Ok(net) => serde_json::to_string_pretty(&json!({
                "name": net.name, "id": net.id, "driver": net.driver, "scope": net.scope,
                "containers": net.containers.as_ref().map(|c| c.keys().collect::<Vec<_>>()),
            })).unwrap(),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Inspect a Docker volume")]
    async fn inspect_volume(&self, Parameters(input): Parameters<VolumeInput>) -> String {
        match self.docker.inspect_volume(&input.name).await {
            Ok(vol) => serde_json::to_string_pretty(&json!({
                "name": vol.name, "driver": vol.driver, "mountpoint": vol.mountpoint,
                "created_at": vol.created_at, "labels": vol.labels,
            })).unwrap(),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Save an image as a tar file")]
    async fn save_image(&self, Parameters(input): Parameters<ExportInput>) -> String {
        let output_path = input.output.unwrap_or(format!("{}.tar", input.id.replace('/', "_").replace(':', "_")));
        let output = tokio::process::Command::new("docker")
            .args(["save", "-o", &output_path, &input.id])
            .output().await;
        match output {
            Ok(o) if o.status.success() => format!("Saved image {} → {}", input.id, output_path),
            Ok(o) => format!("Error: {}", String::from_utf8_lossy(&o.stderr)),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Load an image from a tar file")]
    async fn load_image(&self, Parameters(input): Parameters<LoadInput>) -> String {
        let output = tokio::process::Command::new("docker")
            .args(["load", "-i", &input.path])
            .output().await;
        match output {
            Ok(o) if o.status.success() => format!("Loaded image from {}: {}", input.path, String::from_utf8_lossy(&o.stdout).trim()),
            Ok(o) => format!("Error: {}", String::from_utf8_lossy(&o.stderr)),
            Err(e) => format!("Error: {}", e),
        }
    }
}

adk_mcp_sdk::mcp_2026_server! {
    server: ContainerServer,
    task_tools: ["run_container", "export_container"],
    approval_tools: ["stop_container", "remove_container", "remove_image", "create_network", "create_volume", "restart_container", "remove_network", "remove_volume", "copy_from_container", "copy_to_container", "kill_container", "update_container"],
    cache_ttl_ms: 60_000,
}
