# API Reference

Complete reference for all 42 tools in mcp-containers.

---

## Containers

### list_containers

List running containers. Set `all=true` to include stopped.

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `all` | boolean | ❌ | Include stopped containers (default: false) |

**Example:**
```json
{"name": "list_containers", "arguments": {"all": true}}
```

**Returns:** Array of containers with id, names, image, state, status, ports.

---

### inspect_container

Get full container details.

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `id` | string | ✅ | Container ID or name |

**Returns:** name, image, status, started_at, ports, env vars, mounts.

---

### run_container

Create and start a new container.

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `image` | string | ✅ | Image to run (e.g. `nginx:alpine`) |
| `name` | string | ❌ | Container name |
| `ports` | string[] | ❌ | Port mappings: `["8080:80", "443:443"]` |
| `env` | string[] | ❌ | Environment variables: `["KEY=value"]` |
| `cmd` | string[] | ❌ | Command to run: `["echo", "hello"]` |

**Example:**
```json
{"name": "run_container", "arguments": {"image": "redis:7", "name": "my-redis", "ports": ["6379:6379"]}}
```

---

### stop_container

Gracefully stop a container (sends SIGTERM, waits, then SIGKILL).

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `id` | string | ✅ | Container ID or name |

---

### kill_container

Force kill a container immediately (SIGKILL).

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `id` | string | ✅ | Container ID or name |

---

### remove_container

Remove a stopped container. **Destructive — cannot undo.**

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `id` | string | ✅ | Container ID or name |

---

### restart_container

Stop and start a container.

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `id` | string | ✅ | Container ID or name |

---

### pause_container

Freeze all processes in a container (SIGSTOP).

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `id` | string | ✅ | Container ID or name |

---

### unpause_container

Resume a paused container.

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `id` | string | ✅ | Container ID or name |

---

### rename_container

Rename a container.

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `id` | string | ✅ | Current container ID or name |
| `new_name` | string | ✅ | New name |

---

### get_logs

Get container stdout/stderr output.

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `container_id` | string | ✅ | Container ID or name |
| `tail` | number | ❌ | Number of lines from end (default: 50) |

---

### exec_container

Execute a command inside a running container.

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `container_id` | string | ✅ | Container ID or name |
| `cmd` | string[] | ✅ | Command: `["ls", "-la", "/app"]` |

**Example:**
```json
{"name": "exec_container", "arguments": {"container_id": "api", "cmd": ["cat", "/etc/hostname"]}}
```

---

### get_stats

Get real-time resource usage.

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `id` | string | ✅ | Container ID or name |

**Returns:** memory_mb, memory_limit_mb, memory_pct, cpu_total, network_rx_mb, network_tx_mb.

---

### get_top

Show processes running inside a container.

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `id` | string | ✅ | Container ID or name |

**Returns:** Process table with PID, user, command.

---

### get_changes

Show filesystem changes since container started.

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `id` | string | ✅ | Container ID or name |

**Returns:** Array of `{path, kind}` where kind is 0=modified, 1=added, 2=deleted.

---

### wait_container

Block until container exits, return exit code.

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `id` | string | ✅ | Container ID or name |

---

### update_container

Change resource limits on a running container.

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `id` | string | ✅ | Container ID or name |
| `memory_bytes` | number | ❌ | Memory limit in bytes |
| `cpus` | number | ❌ | CPU limit (e.g. 0.5 = half a core) |

---

## Images

### list_images

List local Docker images.

**Returns:** Array with id, tags, size_mb, created.

---

### pull_image

Pull an image from a registry.

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `image` | string | ✅ | Image name (e.g. `nginx`) |
| `tag` | string | ❌ | Tag (default: `latest`) |

---

### remove_image

Remove a local image. **Destructive.**

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `image` | string | ✅ | Image name |
| `tag` | string | ❌ | Tag (default: `latest`) |

---

### inspect_image

Get image details.

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `image` | string | ✅ | Image name |
| `tag` | string | ❌ | Tag (default: `latest`) |

**Returns:** size_mb, os, architecture, created, env, cmd.

---

### tag_image

Tag an image with a new name.

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `image` | string | ✅ | Source image (e.g. `myapp:latest`) |
| `repo` | string | ✅ | Target repository (e.g. `ghcr.io/org/myapp`) |
| `tag` | string | ✅ | Target tag (e.g. `v2.1.0`) |

---

### image_history

Show layer history of an image.

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `image` | string | ✅ | Image name |
| `tag` | string | ❌ | Tag (default: `latest`) |

---

### save_image

Export an image as a tar file.

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `id` | string | ✅ | Image name:tag |
| `output` | string | ❌ | Output file path (default: `{image}.tar`) |

---

### load_image

Import an image from a tar file.

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `path` | string | ✅ | Path to tar file |

---

## Networks

### list_networks

List all Docker networks.

**Returns:** Array with id, name, driver, scope.

---

### create_network

Create a new network.

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `name` | string | ✅ | Network name |
| `driver` | string | ❌ | Driver (default: `bridge`) |

---

### remove_network

Delete a network. **Destructive.**

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `name` | string | ✅ | Network name or ID |

---

### inspect_network

Get network details including connected containers.

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `name` | string | ✅ | Network name or ID |

---

### connect_network

Connect a container to a network.

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `network` | string | ✅ | Network name |
| `container_id` | string | ✅ | Container to connect |

---

### disconnect_network

Disconnect a container from a network.

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `network` | string | ✅ | Network name |
| `container_id` | string | ✅ | Container to disconnect |

---

## Volumes

### list_volumes

List all Docker volumes.

**Returns:** Array with name, driver, mountpoint.

---

### create_volume

Create a new volume.

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `name` | string | ✅ | Volume name |

---

### remove_volume

Delete a volume. **Destructive — data is lost.**

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `name` | string | ✅ | Volume name |

---

### inspect_volume

Get volume details.

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `name` | string | ✅ | Volume name |

---

## Docker Compose

### compose_up

Start a docker-compose stack in detached mode.

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `path` | string | ❌ | Directory containing compose file (default: `.`) |
| `file` | string | ❌ | Compose filename (default: `docker-compose.yml`) |

---

### compose_down

Stop and remove a docker-compose stack.

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `path` | string | ❌ | Directory containing compose file (default: `.`) |
| `file` | string | ❌ | Compose filename (default: `docker-compose.yml`) |

---

## File Operations

### copy_from_container

Copy a file or directory from a container to the local filesystem.

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `container_id` | string | ✅ | Container ID or name |
| `container_path` | string | ✅ | Path inside container |
| `local_path` | string | ✅ | Destination on host |

---

### copy_to_container

Copy a file or directory from the local filesystem into a container.

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `container_id` | string | ✅ | Container ID or name |
| `container_path` | string | ✅ | Destination inside container |
| `local_path` | string | ✅ | Source on host |

---

## System

### system_info

Get Docker daemon information.

**Returns:** containers count, running, stopped, images, server_version, os, arch, cpus, memory_gb.

---

### prune

Remove all stopped containers, unused images, and dangling volumes. **Destructive.**

**Returns:** containers_removed, images_removed, volumes_removed, space_reclaimed_mb.

---

### export_container

Export a container's filesystem as a tar archive.

| Parameter | Type | Required | Description |
|-----------|------|:---:|-------------|
| `id` | string | ✅ | Container ID or name |
| `output` | string | ❌ | Output file path (default: `{id}.tar`) |
