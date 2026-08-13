# Containers MCP Server

[![Crates.io](https://img.shields.io/crates/v/mcp-containers.svg)](https://crates.io/crates/mcp-containers)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![ADK-Rust Enterprise](https://img.shields.io/badge/ADK--Rust-Enterprise-purple.svg)](https://enterprise.adk-rust.com)
[![Registry Ready](https://img.shields.io/badge/ADK_Registry-Ready-green.svg)](https://www.zavora.ai)

Complete Docker management for AI agents — containers, images, networks, volumes, compose, and system operations via native socket connection. 42 tools with zero configuration.

## Architecture

<p align="center">
  <img src="https://raw.githubusercontent.com/zavora-ai/mcp-containers/main/docs/assets/architecture.svg" alt="MCP Containers Architecture" width="850"/>
</p>

## Documentation

| Document | Description |
|----------|-------------|
| [API Reference](docs/api-reference.md) | All 42 tools with parameters, examples, and return values |
| [Getting Started](docs/getting-started.md) | Installation, first container, common workflows |

## Tools (42)

### Containers (17)

| Tool | Purpose | Risk |
|------|---------|------|
| `list_containers` | List containers (include stopped with `all=true`) | read_only |
| `inspect_container` | Full details: config, mounts, network, state | read_only |
| `run_container` | Run a new container from an image | internal_write |
| `stop_container` | Gracefully stop a container | internal_write |
| `kill_container` | Force kill (SIGKILL) | internal_write |
| `remove_container` | Remove a stopped container | destructive |
| `restart_container` | Restart a container | internal_write |
| `pause_container` | Freeze all processes | internal_write |
| `unpause_container` | Resume a paused container | internal_write |
| `rename_container` | Rename a container | internal_write |
| `get_logs` | Get stdout/stderr logs | read_only |
| `exec_container` | Execute command inside container | internal_write |
| `get_stats` | CPU, memory, network I/O | read_only |
| `get_top` | Running processes inside container | read_only |
| `get_changes` | Filesystem diff (added/modified/deleted) | read_only |
| `wait_container` | Wait for exit, return exit code | read_only |
| `update_container` | Change CPU/memory limits live | internal_write |

### Images (8)

| Tool | Purpose | Risk |
|------|---------|------|
| `list_images` | List local images with sizes | read_only |
| `pull_image` | Pull from registry | internal_write |
| `remove_image` | Remove local image | destructive |
| `inspect_image` | Image details: layers, config, OS | read_only |
| `tag_image` | Tag an image (e.g. `myapp:v2`) | internal_write |
| `image_history` | Show layer history | read_only |
| `save_image` | Export image as tar file | read_only |
| `load_image` | Import image from tar file | internal_write |

### Networks (5)

| Tool | Purpose | Risk |
|------|---------|------|
| `list_networks` | List Docker networks | read_only |
| `create_network` | Create a network | internal_write |
| `remove_network` | Delete a network | destructive |
| `inspect_network` | Network details + connected containers | read_only |
| `connect_network` | Connect container to network | internal_write |
| `disconnect_network` | Disconnect from network | internal_write |

### Volumes (4)

| Tool | Purpose | Risk |
|------|---------|------|
| `list_volumes` | List volumes | read_only |
| `create_volume` | Create a volume | internal_write |
| `remove_volume` | Delete a volume | destructive |
| `inspect_volume` | Volume details + mountpoint | read_only |

### Docker Compose (2)

| Tool | Purpose | Risk |
|------|---------|------|
| `compose_up` | Start a compose stack (`docker compose up -d`) | internal_write |
| `compose_down` | Stop a compose stack (`docker compose down`) | internal_write |

### File Operations (2)

| Tool | Purpose | Risk |
|------|---------|------|
| `copy_from_container` | Copy file out of container | read_only |
| `copy_to_container` | Copy file into container | internal_write |

### System (4)

| Tool | Purpose | Risk |
|------|---------|------|
| `system_info` | Docker version, OS, CPU, memory, counts | read_only |
| `prune` | Remove unused containers, images, volumes | destructive |
| `export_container` | Export container filesystem as tar | read_only |

## Installation

```bash
cargo install mcp-containers
```

## Configuration

**Zero configuration required.** The server auto-connects to your local Docker daemon via:

- macOS: `~/.docker/run/docker.sock`
- Linux: `/var/run/docker.sock`
- Windows: named pipe `//./pipe/docker_engine`
- Custom: set `DOCKER_HOST` env var

### Prerequisites

- Docker Desktop (macOS/Windows) or Docker Engine (Linux) must be running
- No API keys, no tokens, no URLs to configure

## Client Configuration

### Claude Desktop

```json
{
  "mcpServers": {
    "containers": {
      "command": "mcp-containers",
      "args": []
    }
  }
}
```

### Kiro

Add to `.kiro/settings/mcp.json`:

```json
{
  "mcpServers": {
    "containers": {
      "command": "mcp-containers",
      "args": []
    }
  }
}
```

### Cursor

```json
{
  "mcpServers": {
    "containers": {
      "command": "mcp-containers",
      "args": []
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

### Debug a failing service
```
"Why is the API container crashing?"
→ get_logs(container_id="api", tail=50)
→ get_changes(id="api") — see what files changed
→ exec_container(container_id="api", cmd=["cat", "/app/logs/error.log"])
```

### Build and deploy workflow
```
"Tag the current image as v2.1 and push"
→ tag_image(image="myapp:latest", repo="ghcr.io/company/myapp", tag="v2.1")
→ (push via registry)
```

### Manage resources
```
"The API is using too much memory, limit it to 512MB"
→ update_container(id="api", memory_bytes=536870912)
```

### Docker Compose
```
"Start the development stack"
→ compose_up(path="./dev", file="docker-compose.yml")

"Tear it down"
→ compose_down(path="./dev")
```

### Inspect and troubleshoot
```
"What's running on the network?"
→ list_networks()
→ inspect_network(name="my-app-network") — shows connected containers
```

## Tested Live

Every tool has been verified against a real Docker daemon:

```
✅ pull_image: Downloaded alpine:latest
✅ run_container: Started container, executed command
✅ get_logs: Retrieved "hello from mcp-containers!"
✅ pause_container / unpause_container: Freeze/resume
✅ get_top: Shows running processes
✅ get_changes: Filesystem diff (added /tmp/test.txt)
✅ rename_container: Renamed successfully
✅ restart_container: Restarted
✅ kill_container: Force killed
✅ remove_container: Cleaned up
```

## Governance

| Risk Level | Tools |
|-----------|-------|
| **read_only** | list, inspect, logs, stats, top, changes, wait, history, save, copy_from |
| **internal_write** | run, stop, restart, pause, unpause, rename, exec, pull, tag, load, create, connect, compose, copy_to, update |
| **destructive** | remove_container, remove_image, remove_network, remove_volume, kill, prune |

## How It Works

```
MCP Client (Claude/Kiro/Cursor)
    ↓ stdio (JSON-RPC)
mcp-containers (Rust binary)
    ↓ Unix socket (bollard crate)
Docker Daemon
    ↓
Containers, Images, Networks, Volumes
```

No HTTP proxy, no CLI wrapper — direct socket communication via the [bollard](https://crates.io/crates/bollard) crate for maximum performance and reliability.

## License

Apache-2.0

---

Part of the [ADK-Rust Enterprise](https://enterprise.adk-rust.com) MCP server ecosystem.

Built with ❤️ by [Zavora AI](https://zavora.ai)

## rmcp and MCP compatibility

This server is built with [`rmcp` 3.1.2](https://github.com/modelcontextprotocol/rust-sdk/releases/tag/rmcp-v3.1.2) and requires Rust 1.94.1 or newer. The rmcp 3 rollout retains legacy MCP initialization compatibility and targets MCP protocol revisions `2025-11-25` and `2026-07-28`.

## MCP 2026-07-28 rollout (P2 high-impact)

This server uses `rmcp` 3.1.2 and `adk-mcp-sdk` 0.2 with a minimum supported
Rust version of **1.94.1**. It accepts stateless MCP 2026 requests with
per-request protocol, client identity, and capability metadata while retaining
the legacy MCP 2025-11-25 initialize flow for ordinary tools.

- **Tasks:** `run_container`, `export_container`
- **MRTR approvals:** `stop_container`, `remove_container`, `remove_image`, `create_network`, `create_volume`, `restart_container`, `remove_network`, `remove_volume`, `copy_from_container`, `copy_to_container`, `kill_container`, `update_container`
- **Discovery and routing:** rmcp serves on-demand discovery and validates the
  per-request protocol envelope; HTTP deployments can route with `Mcp-Method`
  and `Mcp-Name`. The packaged binary currently uses stdio.
- **Caching:** `tools/list` returns a public `ttlMs` of 60,000 for MCP 2026;
  rmcp omits the cache fields for legacy clients.
- **Deprecated extensions:** this server does not add new Roots, Sampling, or
  dynamic client-registration dependencies.

Protected tools require `MCP_REQUEST_STATE_KEY` with at least 32 high-entropy
bytes. All replicas must share that key so sealed approval state can resume on
another instance. Approval state is bound to the client identity, tool, and
arguments and expires after two minutes. Missing identity, invalid state,
rejection, or legacy protocol use fails closed. Task records are process-local
for the current stdio runtime; use a durable task store before deploying the
server behind scale-to-zero HTTP infrastructure.
