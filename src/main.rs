#![allow(dead_code)]

mod auth;
mod banner;
mod cluster;
mod config;
mod engines;
mod server;
mod storage;
mod web;

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;

use auth::AuthManager;
use banner::print_startup_banner;
use cluster::RaftClusterManager;
use config::AppConfig;
use engines::EngineRegistry;
use server::model_controller::AppState;
use server::{create_router, ServerContext};
use storage::{BackupManager, IdGenerator, LsmBTreeHybrid};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing JettraRDB (Rust Database)...");

    // 1. Load Configuration
    let cfg = AppConfig::load();

    // 2. Storage directory verification & creation
    if !cfg.data_dir.exists() {
        println!(
            "Data directory does not exist. Creating automatically: {:?}",
            cfg.data_dir
        );
        let _ = std::fs::create_dir_all(&cfg.data_dir);
    }

    // 3. Initialize Storage Core
    let storage = Arc::new(LsmBTreeHybrid::new(&cfg.data_dir));
    let backup_mgr = Arc::new(BackupManager::new(&cfg.data_dir));

    // Auto-restore if requested
    if cfg.restore_auto {
        println!("Auto-restore is enabled. Attempting to restore latest backup...");
        backup_mgr.restore_latest_backup();
    }

    // Auto-backup background task
    if cfg.backup_enabled {
        let bm = backup_mgr.clone();
        let interval_mins = cfg.backup_interval_minutes;
        println!(
            "Auto-backup enabled. Running every {} minutes.",
            interval_mins
        );
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(interval_mins * 60));
            loop {
                interval.tick().await;
                let _ = bm.create_backup();
            }
        });
    }

    // 4. Initialize Core Subsystems
    let auth = Arc::new(AuthManager::new());
    let id_gen = Arc::new(IdGenerator::new());
    let raft = Arc::new(RaftClusterManager::new(
        &cfg.node_id,
        cfg.grpc_port,
        &cfg.cluster_peers,
    ));

    // Start Raft cluster listener
    raft.clone().start_listener().await;

    // 5. Initialize the 9 Multi-Model Engines
    let engines = Arc::new(EngineRegistry::new(
        storage.clone(),
        raft.clone(),
        id_gen.clone(),
    ));

    let app_state = Arc::new(AppState {
        auth: auth.clone(),
        engines: engines.clone(),
    });

    // 6. Build Server Context and Router
    let server_ctx = ServerContext {
        storage: storage.clone(),
        app_state,
        backup_mgr,
        cluster: raft.clone(),
        node_id: cfg.node_id.clone(),
        rest_port: cfg.rest_port,
        gui_port: cfg.gui_port,
        grpc_port: cfg.grpc_port,
        peers: cfg.cluster_peers.clone(),
    };

    let router = create_router(server_ctx.clone());

    // 7. Start Network Servers
    let rest_addr = SocketAddr::from(([0, 0, 0, 0], cfg.rest_port));
    let rest_listener = match TcpListener::bind(rest_addr).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to bind REST database port {}: {}", cfg.rest_port, e);
            return Err(e.into());
        }
    };

    // REST server task
    let rest_router = router.clone();
    tokio::spawn(async move {
        let _ = axum::serve(rest_listener, rest_router).await;
    });

    // GUI Web Console server task (if port is distinct)
    if cfg.gui_port != cfg.rest_port {
        let gui_addr = SocketAddr::from(([0, 0, 0, 0], cfg.gui_port));
        if let Ok(gui_listener) = TcpListener::bind(gui_addr).await {
            let gui_router = router.clone();
            tokio::spawn(async move {
                let _ = axum::serve(gui_listener, gui_router).await;
            });
        }
    }

    // 8. Print banner
    print_startup_banner(&cfg);

    // 9. Graceful shutdown handler
    tokio::signal::ctrl_c().await?;
    println!("\nShutdown signal received. Flushing storage and shutting down JettraRDB...");
    storage.close();
    println!("JettraRDB stopped cleanly. Goodbye!");

    Ok(())
}

