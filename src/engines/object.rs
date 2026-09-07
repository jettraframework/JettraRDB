use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::{json, Value};

use crate::cluster::RaftClusterManager;
use crate::storage::LsmBTreeHybrid;

pub struct ObjectEngine {
    storage: Arc<LsmBTreeHybrid>,
    raft: Arc<RaftClusterManager>,
}

impl ObjectEngine {
    pub fn new(storage: Arc<LsmBTreeHybrid>, raft: Arc<RaftClusterManager>) -> Self {
        Self { storage, raft }
    }

    pub fn save_object(&self, collection: &str, obj_id: &str, class_name: &str, state: Value) {
        let key = format!("obj:{}:{}", collection, obj_id);
        let doc = json!({
            "_class": class_name,
            "state": state
        });

        let json_str = doc.to_string();
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

    pub fn get_object(&self, collection: &str, obj_id: &str) -> Option<Value> {
        let key = format!("obj:{}:{}", collection, obj_id);
        if let Some(bytes) = self.storage.get(&key) {
            return serde_json::from_slice(&bytes).ok();
        }
        None
    }

    pub fn delete_object(&self, collection: &str, obj_id: &str) {
        let key = format!("obj:{}:{}", collection, obj_id);
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

    pub fn list(&self, collection: &str) -> HashMap<String, Value> {
        let prefix = format!("obj:{}:", collection);
        let raw = self.storage.scan_prefix(&prefix);
        let mut objs = HashMap::new();
        for (k, v) in raw {
            let oid = k[prefix.len()..].to_string();
            if let Ok(val) = serde_json::from_slice(&v) {
                objs.insert(oid, val);
            }
        }
        objs
    }
}

