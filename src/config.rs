use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub node_id: String,
    pub data_dir: PathBuf,
    pub rest_port: u16,
    pub gui_port: u16,
    pub grpc_port: u16,
    pub cluster_peers: String,
    pub backup_enabled: bool,
    pub backup_interval_minutes: u64,
    pub restore_auto: bool,
}

impl AppConfig {
    pub fn load() -> Self {
        let mut props = HashMap::new();

        // 1. Try to read jettrardb.properties or jettrastoreengine.properties
        let candidate_files = ["jettrardb.properties", "jettrastoreengine.properties"];
        for file_name in candidate_files {
            if let Ok(file) = File::open(file_name) {
                let reader = BufReader::new(file);
                for line in reader.lines().flatten() {
                    let trimmed = line.trim();
                    if trimmed.is_empty() || trimmed.starts_with('#') {
                        continue;
                    }
                    if let Some(pos) = trimmed.find('=') {
                        let key = trimmed[..pos].trim().to_string();
                        let val = trimmed[pos + 1..].trim().to_string();
                        props.insert(key, val);
                    }
                }
                break;
            }
        }

        // Helper to resolve property from ENV first, then properties file, then default
        let get_prop = |key: &str, env_var: &str, default_val: &str| -> String {
            if let Ok(env_val) = std::env::var(env_var) {
                if !env_val.trim().is_empty() {
                    return env_val.trim().to_string();
                }
            }
            if let Some(prop_val) = props.get(key) {
                if !prop_val.trim().is_empty() {
                    return prop_val.trim().to_string();
                }
            }
            default_val.to_string()
        };

        let node_id = get_prop("jettra.node.id", "JETTRA_NODE_ID", "node1");
        let raw_data_dir = get_prop("jettra.data.dir", "JETTRA_DATA_DIR", "./data");
        
        let data_dir = if raw_data_dir.starts_with("~/") {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            PathBuf::from(home).join(&raw_data_dir[2..])
        } else {
            PathBuf::from(raw_data_dir)
        };

        let rest_port: u16 = get_prop("jettra.node.port", "JETTRA_DB_PORT", "8086")
            .parse()
            .unwrap_or(8086);
        let gui_port: u16 = get_prop("jettra.gui.port", "JETTRA_GUI_PORT", "50050")
            .parse()
            .unwrap_or(50050);
        let grpc_port: u16 = get_prop("jettra.grpc.port", "JETTRA_GRPC_PORT", "50051")
            .parse()
            .unwrap_or(50051);

        let cluster_peers = get_prop(
            "jettra.cluster.peers",
            "JETTRA_CLUSTER_PEERS",
            &format!("127.0.0.1:{}", grpc_port),
        );

        let backup_enabled: bool = get_prop("store.backup.enabled", "STORE_BACKUP_ENABLED", "false")
            .parse()
            .unwrap_or(false);
        let backup_interval_minutes: u64 = get_prop(
            "store.backup.interval.minutes",
            "STORE_BACKUP_INTERVAL_MINUTES",
            "60",
        )
        .parse()
        .unwrap_or(60);

        let restore_auto: bool = get_prop("store.restore.auto", "STORE_RESTORE_AUTO", "false")
            .parse()
            .unwrap_or(false);

        Self {
            node_id,
            data_dir,
            rest_port,
            gui_port,
            grpc_port,
            cluster_peers,
            backup_enabled,
            backup_interval_minutes,
            restore_auto,
        }
    }
}

