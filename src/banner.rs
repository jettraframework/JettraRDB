use crate::config::AppConfig;

pub fn print_startup_banner(cfg: &AppConfig) {
    println!(r#"
==================================================================================
                 🦀 JETTRA RDB - HIGH-DENSITY RUST STORAGE ENGINE                 
==================================================================================
  • Node Identifier (jettra.node.id):         {}
  • Storage Directory (jettra.data.dir):      {}
  • REST Database Port (jettra.node.port):    {}
  • Web Management Port (jettra.gui.port):    {}
  • Raft Consensus Port (jettra.grpc.port):   {}
  • Cluster Peers (jettra.cluster.peers):     {}
  • Auto-Restore (store.restore.auto):        {}
  • Auto-Backup (store.backup.enabled):       {} (Interval: {} min)
  • Memory Model:                             Zero-GC Native Rust (Deterministic)
  --------------------------------------------------------------------------------
  [Web Management & Console URLs]:
  • Web Management UI (GUI):                  http://localhost:{}/ (or /dashboard)
  • Multi-Model Database Engines:             http://localhost:{}/engines
  • Engines Architecture Information:         http://localhost:{}/information
  • Users & Security (Per-Database RBAC):     http://localhost:{}/users
  • Cluster Topology & Internals:             http://localhost:{}/components
  • Swagger OpenAPI Explorer:                 http://localhost:{}/swagger-ui
  --------------------------------------------------------------------------------
  [REST Database APIs]:
  • REST Universal Multi-Model API:           http://localhost:{}/api/model/
  • REST Document Engine API:                 http://localhost:{}/api/document/
  • Default Admin Credentials:                admin / admin  (or super-user / superUserZ)
==================================================================================
"#,
        cfg.node_id,
        cfg.data_dir.display(),
        cfg.rest_port,
        cfg.gui_port,
        cfg.grpc_port,
        cfg.cluster_peers,
        cfg.restore_auto,
        cfg.backup_enabled,
        cfg.backup_interval_minutes,
        cfg.gui_port,
        cfg.gui_port,
        cfg.gui_port,
        cfg.gui_port,
        cfg.gui_port,
        cfg.gui_port,
        cfg.rest_port,
        cfg.rest_port
    );
}

