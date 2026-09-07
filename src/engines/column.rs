use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::Value;

use crate::cluster::RaftClusterManager;
use crate::storage::LsmBTreeHybrid;

pub struct ColumnEngine {
    storage: Arc<LsmBTreeHybrid>,
    raft: Arc<RaftClusterManager>,
}

impl ColumnEngine {
    pub fn new(storage: Arc<LsmBTreeHybrid>, raft: Arc<RaftClusterManager>) -> Self {
        Self { storage, raft }
    }

    pub fn insert_row(&self, column_family: &str, row_key: &str, columns: Value) {
        let key = format!("col:{}:{}", column_family, row_key);
        let json_str = columns.to_string();
        let now_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64;

        self.storage.put(&key, json_str.as_bytes(), now_ms);

        let cmd = format!("PUT {} {}", key, json_str);
        let raft = self.raft.clone();
        tokio::spawn(async move {
            let _ = raft.replicate_command(&cmd).await;
        });
    }

    pub fn get_row(&self, column_family: &str, row_key: &str) -> Option<Value> {
        let key = format!("col:{}:{}", column_family, row_key);
        if let Some(bytes) = self.storage.get(&key) {
            return serde_json::from_slice(&bytes).ok();
        }
        None
    }

    pub fn delete_row(&self, column_family: &str, row_key: &str) {
        let key = format!("col:{}:{}", column_family, row_key);
        let now_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64;

        self.storage.delete(&key, now_ms);

        let cmd = format!("DELETE {}", key);
        let raft = self.raft.clone();
        tokio::spawn(async move {
            let _ = raft.replicate_command(&cmd).await;
        });
    }

    pub fn list(&self, column_family: &str) -> HashMap<String, Value> {
        let prefix = format!("col:{}:", column_family);
        let raw = self.storage.scan_prefix(&prefix);
        let mut rows = HashMap::new();
        for (k, v) in raw {
            let rk = k[prefix.len()..].to_string();
            if let Ok(val) = serde_json::from_slice(&v) {
                rows.insert(rk, val);
            }
        }
        rows
    }
}

