# JettraRDB - Docker & Container Deployment Guide

This guide details how to build, run, and orchestrate **JettraRDB** using Docker and Docker Compose.

---

## 1. Container Images Overview

JettraRDB provides two production-ready Dockerfile configurations:

| Dockerfile | Base Image | Size | Description |
| :--- | :--- | :--- | :--- |
| **`Dockerfile`** | `debian:bookworm-slim` | ~65 MB | Standard production image with glibc and CA certificates. |
| **`Dockerfile.alpine`** | `alpine:3.21` | **< 20 MB** | Ultra-compact image with statically linked musl binary. |

---

## 2. Building Docker Images

### 2.1 Standard Debian Slim Image
```bash
docker build -t jettra/rdb:latest .
```

### 2.2 Ultra-Compact Alpine Static Image
```bash
docker build -f Dockerfile.alpine -t jettra/rdb:alpine .
```

### 2.3 Multi-Architecture Builds (AMD64 + ARM64)
To build multi-arch images for Intel/AMD servers and ARM64 / Apple Silicon:
```bash
docker buildx create --use
docker buildx build --platform linux/amd64,linux/arm64 -t jettra/rdb:latest --push .
```

---

## 3. Running a Standalone Container

Run a single node of JettraRDB mounting a local data volume:

```bash
docker run -d \
  --name jettrardb \
  -p 8086:8086 \
  -p 50050:50050 \
  -p 50051:50051 \
  -v $(pwd)/data:/data \
  -e JETTRA_NODE_ID=node1 \
  -e JETTRA_DATA_DIR=/data \
  -e JETTRA_DB_PORT=8086 \
  -e JETTRA_GUI_PORT=50050 \
  -e JETTRA_GRPC_PORT=50051 \
  --restart unless-stopped \
  jettra/rdb:latest
```

### Exposed Ports:
- **`8086`**: REST Database Universal API (`http://localhost:8086/api/model/`)
- **`50050`**: JettraFlux Web Management Console (`http://localhost:50050/`)
- **`50051`**: Raft Consensus & Peer Replication

---

## 4. Orchestrating a 3-Node Raft Cluster (Docker Compose)

The included `docker-compose.yml` configures a production-grade 3-node distributed cluster with automated quorum and peer discovery over an isolated bridge network.

### 4.1 Cluster Architecture
```
                                Client / Traffic
                                       │
            ┌──────────────────────────┼──────────────────────────┐
            ▼                          ▼                          ▼
     [jettrardb-node1]          [jettrardb-node2]          [jettrardb-node3]
     REST Port: 8086            REST Port: 8087            REST Port: 8088
     GUI Port:  50050           GUI Port:  50052           GUI Port:  50054
     Raft Port: 50051           Raft Port: 50053           Raft Port: 50055
            ▲                          ▲                          ▲
            └────────── Raft Cluster Network (50051) ─────────────┘
```

### 4.2 Start the Cluster
```bash
docker compose up -d --build
```

### 4.3 Check Cluster Status
```bash
docker compose ps
```

### 4.4 View Live Cluster Logs
```bash
docker compose logs -f
```

### 4.5 Stop the Cluster
```bash
docker compose down
```

To stop the cluster and delete persistent storage volumes:
```bash
docker compose down -v
```

---

## 5. Configuration Environment Variables

You can configure any node via Docker environment variables:

| Variable | Default | Description |
| :--- | :--- | :--- |
| `JETTRA_NODE_ID` | `node1` | Unique node identifier in the cluster |
| `JETTRA_DATA_DIR` | `/data` | Directory where storage files (`wal.jettra`, `data_0.jettra`) are stored |
| `JETTRA_DB_PORT` | `8086` | Port for the REST Database API |
| `JETTRA_GUI_PORT` | `50050` | Port for the JettraFlux Web Console |
| `JETTRA_GRPC_PORT` | `50051` | Port for Raft consensus TCP communication |
| `JETTRA_CLUSTER_PEERS` | `127.0.0.1:50051` | Comma-separated list of peer `host:port` addresses |
| `STORE_BACKUP_ENABLED` | `false` | Enable automatic background snapshot backups |
| `STORE_BACKUP_INTERVAL_MINUTES` | `60` | Backup interval in minutes |
| `STORE_RESTORE_AUTO` | `false` | Automatically restore the latest snapshot on container boot |

---

## 6. Container Health Verification

Check container health and test query execution:
```bash
# Verify health endpoint
curl http://localhost:8086/api/system/health

# Run test suite against Node 1
./test_all_models.sh http://localhost:8086

# Run test suite against Node 2
./test_all_models.sh http://localhost:8087
```

