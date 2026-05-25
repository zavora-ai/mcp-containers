# Containers MCP Server

[![Crates.io](https://img.shields.io/crates/v/mcp-containers.svg)](https://crates.io/crates/mcp-containers)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![ADK-Rust Enterprise](https://img.shields.io/badge/ADK--Rust-Enterprise-purple.svg)](https://enterprise.adk-rust.com)

Container management for AI agents — run, stop, exec, logs, images, builds, registries, vulnerability scanning, and Kubernetes pods. 22 tools with Docker, Kubernetes, and registry backends.

## Tools (22)

### Containers (7)

| Tool | Purpose | Risk |
|------|---------|------|
| `list_containers` | List running containers | read_only |
| `get_container` | Container details + resource usage | read_only |
| `run_container` | Run new container from image | internal_write |
| `stop_container` | Stop a running container | internal_write |
| `remove_container` | Remove stopped container | destructive |
| `get_container_logs` | Container stdout/stderr | read_only |
| `exec_in_container` | Execute command inside container | internal_write |

### Images (5)

| Tool | Purpose | Risk |
|------|---------|------|
| `list_images` | List local images | read_only |
| `pull_image` | Pull from registry | internal_write |
| `build_image` | Build from Dockerfile | internal_write |
| `push_image` | Push to registry | external_write |
| `remove_image` | Remove local image | destructive |

### Registry (4)

| Tool | Purpose | Risk |
|------|---------|------|
| `list_repositories` | List repos in registry | read_only |
| `list_tags` | List tags for a repo | read_only |
| `get_manifest` | Image manifest + layers | read_only |
| `scan_image` | Vulnerability scan | read_only |

### Kubernetes Pods (4)

| Tool | Purpose | Risk |
|------|---------|------|
| `list_pods` | List pods in namespace | read_only |
| `get_pod` | Pod details + events | read_only |
| `delete_pod` | Delete pod (triggers reschedule) | destructive |
| `get_pod_logs` | Pod logs | read_only |

### System (2)

| Tool | Purpose | Risk |
|------|---------|------|
| `get_system_info` | Docker version, OS, resources | read_only |
| `prune` | Remove unused containers/images/volumes | destructive |

## Installation

```bash
cargo install mcp-containers
```

## Configuration

### Runtime (pick one)

| Backend | Env Vars |
|---------|----------|
| **Docker Engine** | `DOCKER_HOST` (e.g. `tcp://localhost:2375` or `unix:///var/run/docker.sock`) |
| **Kubernetes** | `KUBERNETES_API_URL` + `KUBERNETES_TOKEN` |
| **Custom API** | `CONTAINERS_API_URL` + `CONTAINERS_API_KEY` |

### Registry (optional)

| Backend | Env Vars |
|---------|----------|
| **Docker Hub** | `DOCKERHUB_TOKEN` |
| **GitHub (ghcr.io)** | `GHCR_TOKEN` |
| **AWS ECR** | `ECR_TOKEN` + `ECR_REGISTRY` |

## Client Configuration

```json
{
  "mcpServers": {
    "containers": {
      "command": "mcp-containers",
      "args": [],
      "env": {
        "DOCKER_HOST": "unix:///var/run/docker.sock",
        "GHCR_TOKEN": "ghp_xxxxx"
      }
    }
  }
}
```

## Usage Examples

### Run a container
```
"Start a Redis container on port 6379"
→ run_container(image="redis:7", name="my-redis", ports=["6379:6379"])
```

### Debug a failing container
```
"Why is the API container crashing?"
→ get_container_logs(container_id="api-1", lines=50)
→ exec_in_container(container_id="api-1", command=["cat", "/app/logs/error.log"])
```

### Build and push
```
"Build and push the new API image"
→ build_image(context=".", tag="ghcr.io/company/api:v2.1")
→ push_image(image="ghcr.io/company/api", tag="v2.1")
```

### Security scan
```
"Scan the production image for vulnerabilities"
→ scan_image(image="company/api", tag="latest")
```

## License

Apache-2.0 — Part of [ADK-Rust Enterprise](https://enterprise.adk-rust.com)
