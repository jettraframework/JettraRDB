use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::Value;

use crate::cluster::RaftClusterManager;
use crate::storage::{IdGenerator, IdMode, LsmBTreeHybrid, RecordVersion};

pub struct DocumentEngine {
    storage: Arc<LsmBTreeHybrid>,
    raft: Arc<RaftClusterManager>,
    id_gen: Arc<IdGenerator>,
}

impl DocumentEngine {
    pub fn new(
        storage: Arc<LsmBTreeHybrid>,
        raft: Arc<RaftClusterManager>,
        id_gen: Arc<IdGenerator>,
    ) -> Self {
        Self {
            storage,
            raft,
            id_gen,
        }
    }

    pub fn insert(
        &self,
        collection: &str,
        document_id: Option<&str>,
        document: Value,
        id_mode: IdMode,
    ) -> String {
        let resolved_id = self.id_gen.generate_id(collection, id_mode, document_id);
        let key = format!("doc:{}:{}", collection, resolved_id);
        let json_str = serde_json::to_string(&document).unwrap_or_else(|_| "{}".to_string());
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

        resolved_id
    }

    pub fn get(&self, collection: &str, document_id: &str) -> Option<Value> {
        let key = format!("doc:{}:{}", collection, document_id);
        if let Some(bytes) = self.storage.get(&key) {
            if let Ok(v) = serde_json::from_slice::<Value>(&bytes) {
                return Some(v);
            }
        }
        // Fallback: search key ending with :document_id in collection
        let prefix = format!("doc:{}:", collection);
        let scanned = self.storage.scan_prefix(&prefix);
        for (k, v) in scanned {
            if k.ends_with(&format!(":{}", document_id)) {
                if let Ok(val) = serde_json::from_slice::<Value>(&v) {
                    return Some(val);
                }
            }
        }
        None
    }

    pub fn delete(&self, collection: &str, document_id: &str) {
        let key = format!("doc:{}:{}", collection, document_id);
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

    pub fn get_history(&self, collection: &str, document_id: &str) -> Vec<RecordVersion> {
        let key = format!("doc:{}:{}", collection, document_id);
        self.storage.get_version_history(&key)
    }

    pub fn restore_version(&self, collection: &str, document_id: &str, timestamp: i64) -> bool {
        let key = format!("doc:{}:{}", collection, document_id);
        self.storage.restore_version(&key, timestamp)
    }

    pub fn list(&self, collection: &str) -> HashMap<String, Value> {
        let prefix = format!("doc:{}:", collection);
        let raw = self.storage.scan_prefix(&prefix);
        let mut docs = HashMap::new();
        for (k, v) in raw {
            let doc_id = k[prefix.len()..].to_string();
            if let Ok(val) = serde_json::from_slice::<Value>(&v) {
                docs.insert(doc_id, val);
            }
        }
        docs
    }
}

