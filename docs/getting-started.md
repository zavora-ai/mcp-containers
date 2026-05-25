# Getting Started

## Prerequisites

1. **Docker** must be installed and running
   - macOS/Windows: [Docker Desktop](https://www.docker.com/products/docker-desktop/)
   - Linux: `sudo apt install docker.io` or [Docker Engine](https://docs.docker.com/engine/install/)

2. **Verify Docker is running:**
   ```bash
   docker version
   ```

## Install

```bash
cargo install mcp-containers
```

Or build from source:
```bash
git clone https://github.com/zavora-ai/mcp-containers
cd mcp-containers
cargo build --release
```

## First Run

```bash
mcp-containers
```

That's it. No env vars, no config files. The server connects to Docker automatically and exposes 42 tools via MCP.

## Your First Container

Connect via any MCP client (Claude Desktop, Kiro, Cursor) and try:

### 1. Check Docker is connected
```
"What's the Docker system info?"
→ system_info()
```

Response:
```json
{
  "containers": 3,
  "running": 1,
  "images": 12,
  "server_version": "29.1.3",
  "os": "Docker Desktop",
  "cpus": 10,
  "memory_gb": 16
}
```

### 2. Pull and run a container
```
"Run an nginx web server on port 8080"
→ pull_image(image="nginx", tag="alpine")
→ run_container(image="nginx:alpine", name="my-web", ports=["8080:80"])
```

### 3. Check it's running
```
"Is the web server running?"
→ list_containers()
```

### 4. View logs
```
"Show me the nginx logs"
→ get_logs(container_id="my-web", tail=20)
```

### 5. Clean up
```
"Stop and remove the web server"
→ stop_container(id="my-web")
→ remove_container(id="my-web")
```

## Common Workflows

### Development Environment

```
"Start my dev stack"
→ compose_up(path="/path/to/project")

"Check what's running"
→ list_containers()

"View API logs"
→ get_logs(container_id="project-api-1", tail=50)

"Restart after code change"
→ restart_container(id="project-api-1")

"Tear it all down"
→ compose_down(path="/path/to/project")
```

### Debugging a Container

```
"The API container keeps crashing"
→ inspect_container(id="api") — check status, exit code
→ get_logs(container_id="api", tail=100) — see error output
→ get_changes(id="api") — what files were modified?
→ exec_container(container_id="api", cmd=["ls", "-la", "/app"]) — inspect filesystem
```

### Image Management

```
"What images do I have locally?"
→ list_images()

"Pull the latest postgres"
→ pull_image(image="postgres", tag="16-alpine")

"Tag for our registry"
→ tag_image(image="myapp:latest", repo="ghcr.io/company/myapp", tag="v2.1.0")

"Clean up old images"
→ prune()
```

### Network Troubleshooting

```
"What networks exist?"
→ list_networks()

"Which containers are on the app network?"
→ inspect_network(name="myapp_default")

"Connect the debug container to the app network"
→ connect_network(network="myapp_default", container_id="debug-tools")
```

### Resource Management

```
"How much memory is the DB using?"
→ get_stats(id="postgres")

"Limit it to 1GB"
→ update_container(id="postgres", memory_bytes=1073741824)

"What processes are running inside?"
→ get_top(id="postgres")
```

## Troubleshooting

### "Docker is not running"

Make sure Docker Desktop is started (macOS/Windows) or the daemon is running (Linux):
```bash
# Linux
sudo systemctl start docker

# macOS — open Docker Desktop app
```

### "Permission denied"

On Linux, your user needs to be in the `docker` group:
```bash
sudo usermod -aG docker $USER
# Then log out and back in
```

### Custom Docker Host

If Docker is on a remote machine:
```bash
DOCKER_HOST=tcp://192.168.1.100:2375 mcp-containers
```

Or via SSH:
```bash
DOCKER_HOST=ssh://user@remote-host mcp-containers
```

## Next Steps

- Read the [API Reference](api-reference.md) for all 42 tools
- Try the [Docker Compose workflow](#development-environment) with your project
- Connect to your CI/CD pipeline with `mcp-cicd` + `mcp-containers`
