# JettraRDB - Developer & Architecture Guide

Welcome to the **JettraRDB** development guide. This document details the internal design, concurrency patterns, storage engine mechanics, and instructions for contributing or extending the database with new capabilities.

---

## 1. Project Organization & Architecture

The codebase is organized into modular components adhering to Rust best practices:

```
JettraRDB/
├── Cargo.toml                  # Package definition & compiler optimization profiles
├── jettrardb.properties        # Default runtime properties
├── Dockerfile                  # Production Debian-slim multi-stage build
├── Dockerfile.alpine           # Ultra-compact static Musl build
├── docker-compose.yml          # 3-node distributed Raft cluster configuration
├── test_all_models.sh          # Full automated end-to-end verification script
├── build.sh                    # Build helper script
├── src/
│   ├── main.rs                 # Bootstrap, CLI flags, graceful shutdown
│   ├── config.rs               # Property parser (.properties & ENV overrides)
│   ├── banner.rs               # ASCII banner & interactive endpoints report
│   ├── storage/                # Core Storage Engine
│   │   ├── mod.rs
│   │   ├── lsm_btree_hybrid.rs # Hybrid LSM + B-Tree, WAL, MemTable, MVCC history
│   │   ├── file_manager.rs     # Low-level SSTable block file I/O
│   │   ├── id_generator.rs     # Multi-mode ID generator (Manual, Auto, UUID)
│   │   └── backup_manager.rs   # Hot snapshot backups & auto-restore
│   ├── engines/                # The 9 Multi-Model Database Engines
│   │   ├── mod.rs              # Central EngineRegistry
│   │   ├── document.rs         # NoSQL JSON documents & PITR restoration
│   │   ├── vector.rs           # AI vector embeddings & Cosine similarity
│   │   ├── graph.rs            # Labeled Property Graph (LPG) & traversal
│   │   ├── timeseries.rs       # IoT telemetry & temporal ranges
│   │   ├── column.rs           # OLAP columnar storage
│   │   ├── keyvalue.rs         # In-memory atomic MemTable key-value
│   │   ├── geospatial.rs       # 2D GIS & Haversine distance
│   │   ├── object.rs           # Chunked BLOBs & class metadata
│   │   └── records.rs          # Strongly typed schema records & projections
│   ├── auth/                   # Security & RBAC
│   │   ├── mod.rs
│   │   └── auth_manager.rs     # Users, credentials, Bearer tokens
│   ├── cluster/                # Clustering & Consensus
│   │   ├── mod.rs
│   │   └── raft.rs             # Raft consensus manager & TCP replication
│   ├── server/                 # Network & REST APIs
│   │   ├── mod.rs
│   │   ├── routes.rs           # Axum router & CORS middleware
│   │   ├── auth_controller.rs  # Authentication endpoints
│   │   ├── model_controller.rs # Universal /api/model endpoints
│   │   ├── document_controller.rs # Document CRUD & version history endpoints
│   │   └── backup_controller.rs # Backup trigger endpoint
│   └── web/                    # JettraFlux Web Console
│       ├── mod.rs
│       └── templates.rs        # Embedded HTML5/Bootstrap responsive pages
└── docs/
    ├── USAGE_GUIDE.md          # Comprehensive REST API usage guide
    ├── EXECUTABLES_GUIDE.md    # Compilation & standalone binaries guide
    ├── DOCKER_GUIDE.md         # Docker & Compose deployment guide
    └── DEVELOPMENT_GUIDE.md    # This development guide
```

---

## 2. Storage Engine Internals

### 2.1 Per-Database Isolation
Unlike monolithic databases that mix all databases in one giant table, `LsmBTreeHybrid` partitions storage by database:
- `data/databases/<dbName>/wal.jettra`: Append-only Write-Ahead Log.
- `data/databases/<dbName>/data_0.jettra`: SSTable data file.
- `data/system/`: System and administrative partitions.

Operations on database `FinanceDB` are strictly isolated from `TelemetryDB`, eliminating write locks across tenants and allowing zero-downtime database drops (`drop_database`).

### 2.2 Ingestion & The Write Pipeline
1. Incoming write arrives via Axum REST controller.
2. The key is evaluated to extract the target database via `LsmBTreeHybrid::extract_database_from_key(key)`.
3. The write is appended sequentially to the database's `wal.jettra`.
4. The write is inserted into the in-memory concurrent `MemTable` (`BTreeMap<String, Vec<u8>>`) under key `key@timestamp`.
5. The write command is asynchronously broadcast to peer nodes via `RaftClusterManager`.
6. When the `MemTable` size exceeds `flush_threshold` (1,000 entries), records are flushed sequentially into the SSTable `data_0.jettra`, and offsets are indexed in `disk_index`.

### 2.3 Point-In-Time-Recovery (PITR) & Document MVCC
Every update in JettraRDB creates a new version tagged with monotonic epoch milliseconds (`key@timestamp`).
- `get_version_history(key)` reads historical revisions in reverse chronological order.
- `restore_version(key, timestamp)` rolls back the active state to the chosen historical snapshot.

---

## 3. How to Add a New Database Model

Suppose you want to add a new **FullText Engine** (`FULLTEXT`):

### Step 1: Create `src/engines/fulltext.rs`
```rust
use std::sync::Arc;
use crate::storage::LsmBTreeHybrid;
use crate::cluster::RaftClusterManager;

pub struct FullTextEngine {
    storage: Arc<LsmBTreeHybrid>,
    raft: Arc<RaftClusterManager>,
}

impl FullTextEngine {
    pub fn new(storage: Arc<LsmBTreeHybrid>, raft: Arc<RaftClusterManager>) -> Self {
        Self { storage, raft }
    }

    pub fn index_text(&self, collection: &str, doc_id: &str, content: &str) {
        let key = format!("ft:{}:{}", collection, doc_id);
        let now_ms = chrono::Utc::now().timestamp_millis();
        self.storage.put(&key, content.as_bytes(), now_ms);
    }

    pub fn search(&self, collection: &str, term: &str) -> Vec<String> {
        let prefix = format!("ft:{}:", collection);
        let mut matches = Vec::new();
        for (k, v) in self.storage.scan_prefix(&prefix) {
            if let Ok(text) = String::from_utf8(v) {
                if text.to_lowercase().contains(&term.to_lowercase()) {
                    matches.push(k);
                }
            }
        }
        matches
    }
}
```

### Step 2: Register in `src/engines/mod.rs`
1. Add `pub mod fulltext;`
2. Add `pub fulltext: Arc<FullTextEngine>` to `EngineRegistry`.
3. Add entry to `list_engines()`.

### Step 3: Map in `src/server/model_controller.rs`
Add `"FULLTEXT"` to the match expressions in `post_model_handler` and `get_model_handler`.

---

## 4. Concurrency & Memory Model

- **Tokio Asynchronous Runtime**: All network I/O (REST HTTP, Raft TCP) runs concurrently on green tasks without thread-pool exhaustion.
- **`parking_lot::RwLock`**: Reader-writer locks provide high-concurrency read operations while ensuring thread-safe write updates to the `MemTable` and `disk_index`.
- **Zero GC Latency**: Memory is freed immediately when dropped, preventing JVM GC stalls.

---

## 5. Development Workflow

### 5.1 Format Code
Ensure code adheres to standard formatting:
```bash
cargo fmt
```

### 5.2 Static Analysis with Clippy
Catch subtle bugs, performance pitfalls, and unidiomatic code:
```bash
cargo clippy -- -D warnings
```

### 5.3 Run Unit Tests
```bash
cargo test
```

### 5.4 Run Automated Integration Tests
Launch JettraRDB:
```bash
cargo run &
JETTRA_PID=$!
sleep 2

# Run full integration suite
./test_all_models.sh

# Stop server
kill $JETTRA_PID
```

