use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::cluster::RaftClusterManager;
use crate::storage::LsmBTreeHybrid;

pub struct KeyValueEngine {
    storage: Arc<LsmBTreeHybrid>,
    raft: Arc<RaftClusterManager>,
}

impl KeyValueEngine {
    pub fn new(storage: Arc<LsmBTreeHybrid>, raft: Arc<RaftClusterManager>) -> Self {
        Self { storage, raft }
    }

    pub fn put(&self, namespace: &str, key: &str, value: &str) {
        let internal_key = format!("kv:{}:{}", namespace, key);
        let now_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64;

        self.storage.put(&internal_key, value.as_bytes(), now_ms);

        let cmd = format!("PUT {} {}", internal_key, value);
        let raft = self.raft.clone();
        tokio::spawn(async move {
            let _ = raft.replicate_command(&cmd).await;
        });
    }

    pub fn get(&self, namespace: &str, key: &str) -> Option<String> {
        let internal_key = format!("kv:{}:{}", namespace, key);
        if let Some(bytes) = self.storage.get(&internal_key) {
            return String::from_utf8(bytes).ok();
        }
        None
    }

    pub fn delete(&self, namespace: &str, key: &str) {
        let internal_key = format!("kv:{}:{}", namespace, key);
        let now_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64;

        self.storage.delete(&internal_key, now_ms);

        let cmd = format!("DELETE {}", internal_key);
        let raft = self.raft.clone();
        tokio::spawn(async move {
            let _ = raft.replicate_command(&cmd).await;
        });
    }

    pub fn list(&self, namespace: &str) -> HashMap<String, String> {
        let prefix = format!("kv:{}:", namespace);
        let raw = self.storage.scan_prefix(&prefix);
        let mut map = HashMap::new();
        for (k, v) in raw {
            let user_key = k[prefix.len()..].to_string();
            if let Ok(s) = String::from_utf8(v) {
                map.insert(user_key, s);
            }
        }
        map
    }
}

