# JettraRDB (Jettra Rust Database)

Autonomous, high-density, multi-model storage engine engineered natively in **Rust** with Tokio asynchronous concurrency, zero Garbage Collection pauses, and memory safety.

`JettraRDB` is the next-generation Rust implementation of **JettraDB**, unifying **9 distinct database models** over a single resilient storage core combining Log-Structured Merge Trees (LSM), B-Trees, and Raft consensus log replication.

---

## 🏛️ Architecture Overview

```
                      JettraRDB (Port 8086 / 50050 / 50051)
   ┌─────────────────────────────────────────────────────────────────────────┐
   │ 1. DOCUMENT   : NoSQL JSON Trees, Auto/UUID IDs & PITR Restoration      │
   │ 2. VECTOR     : AI ANN Vector Embeddings & Cosine Similarity            │
   │ 3. GRAPH      : LPG Nodes, Directed Labeled Edges & Deep Traversal      │
   │ 4. TIMESERIES : High-Frequency IoT Telemetry & Temporal Aggregations    │
   │ 5. COLUMN     : OLAP Columnar Projections & Column Families             │
   │ 6. KEYVALUE   : Atomic In-Memory MemTable Cache & Fast Key-Value        │
   │ 7. GEOSPATIAL : 2D GIS Coordinates & Haversine Distance Queries         │
   │ 8. OBJECT     : Chunked Binary BLOBs, Media Streams & Class Metadata    │
   │ 9. RECORDS    : Strongly Typed Records, Validation & Field Projections  │
   └─────────────────────────────────────────────────────────────────────────┘
                                      │
                   Hybrid LSM-Tree / B-Tree Storage Core
                   (MemTable + wal.jettra + data_0.jettra)
                                      │
                   Raft Consensus & Distributed Peer Replication
```

---

## ⚡ The Rust Advantage over JVM

| Feature | JettraDB (Java 25) | JettraRDB (Rust) |
| :--- | :--- | :--- |
| **Garbage Collection Latency** | ZGC pauses (< 1ms) | **0 ms (Zero GC, deterministic memory)** |
| **Container Image Size** | ~180MB - 350MB (JRE) | **< 20MB (Alpine / Musl static binary)** |
| **Memory Footprint** | Heap allocation + JVM overhead | **Direct memory allocation (Cache-friendly)** |
| **Deployment Simplicity** | Requires JRE / JDK 25 | **Single standalone native executable** |
| **Concurrency Model** | Virtual Threads (Loom) | **Tokio asynchronous runtime** |
| **Type Safety** | Object references & reflection | **Zero-cost compile-time abstractions** |

---

## 🚀 Quick Start

### 1. Build and Run Natively
```bash
# Build release executable
cargo build --release

# Run JettraRDB
./target/release/jettrardb
```

### 2. Default Ports and Services
- **REST Database API**: `http://localhost:8086`
- **JettraFlux Web Console**: `http://localhost:50050`
- **Raft Consensus Port**: `50051`
- **Interactive Swagger / OpenAPI UI**: `http://localhost:50050/swagger-ui`

### 3. Default Administrative Credentials
- **Admin**: `admin` / `admin`
- **Superuser**: `super-user` / `superUserZ`

### 4. Verification Test
```bash
./test_all_models.sh
```

---

## 📚 Comprehensive Guides and Documentation

Detailed documentation manuals are available in the [`docs/`](docs/) directory:

1. 📖 **[Usage Guide (`docs/USAGE_GUIDE.md`)](docs/USAGE_GUIDE.md)**:
   - Full REST API reference for all 9 multi-model engines.
   - Authentication, token management, and per-database RBAC.
   - cURL, JSON payloads, query parameters, and Point-In-Time-Recovery (PITR).

2. ⚙️ **[Executables Generation Guide (`docs/EXECUTABLES_GUIDE.md`)](docs/EXECUTABLES_GUIDE.md)**:
   - Native compilation (`cargo build --release`).
   - Static musl builds without glibc dependencies.
   - Cross-compilation for Linux x86_64 / ARM64, macOS, and Windows (`.exe`).
   - Binary stripping and size optimization.

3. 🐳 **[Docker & Containers Guide (`docs/DOCKER_GUIDE.md`)](docs/DOCKER_GUIDE.md)**:
   - Multi-stage Docker builds.
   - Ultra-compact Alpine/Musl Dockerfile (< 20MB).
   - Multi-node 3-node Raft cluster with `docker-compose.yml`.
   - Volume persistence, network isolation, and production health checks.

4. 🛠️ **[Development Guide (`docs/DEVELOPMENT_GUIDE.md`)](docs/DEVELOPMENT_GUIDE.md)**:
   - Codebase structure and design patterns.
   - Adding a new database engine model step-by-step.
   - Testing, benchmarking with `criterion`, code formatting, and clippy rules.

---

## 📄 License
Licensed under Apache-2.0 or MIT.




Para compilar **JettraRDB** en tu sistema tienes tres alternativas muy sencillas. La **Opción 1** es la recomendada oficialmente por la comunidad de Rust, la **Opción 2** usa el gestor de paquetes de Ubuntu (`apt`), y la **Opción 3** compila directamente mediante Docker sin instalar nada en tu sistema.

---

### Opción 1 (Recomendada): Instalar Rust con `rustup` (Oficial)
Esta opción instala la versión más reciente y estable en tu directorio de usuario (no requiere permisos de `sudo`):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
```

Una vez finalice la instalación, activa las variables en tu terminal actual:
```bash
source "$HOME/.cargo/env"
```

Verifica la instalación:
```bash
cargo --version
rustc --version
```

Ahora ya puedes compilar JettraRDB:
```bash
cd /home/avbravo/NetBeansProjects/jettrastack_local/JettraWorkspace/JettraRDB
cargo build --release
```

---

### Opción 2: Instalar mediante `apt` (Ubuntu)
Si prefieres usar los paquetes del sistema operativo:

```bash
sudo apt update
sudo apt install -y cargo rustc
```

Luego compila:
```bash
cd /home/avbravo/NetBeansProjects/jettrastack_local/JettraWorkspace/JettraRDB
cargo build --release
```

---

### Opción 3: Compilar usando Docker (Sin instalar Rust en el sistema)
Dado que ya tienes Docker instalado, puedes generar el binario dentro de un contenedor oficial de Rust y obtener el ejecutable directamente en tu máquina:

```bash
cd /home/avbravo/NetBeansProjects/jettrastack_local/JettraWorkspace/JettraRDB
docker run --rm -v "$(pwd)":/usr/src/jettrardb -w /usr/src/jettrardb rust:latest cargo build --release
```

El ejecutable compilado quedará en:
```
target/release/jettrardb
```

O si prefieres construir y ejecutar la imagen completa de Docker:
```bash
docker build -t jettra/rdb:latest .
docker run -d -p 8086:8086 -p 50050:50050 --name rdb jettra/rdb:latest
```