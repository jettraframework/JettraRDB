use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::{json, Map, Value};

use crate::cluster::RaftClusterManager;
use crate::storage::LsmBTreeHybrid;

pub struct RecordsEngine {
    storage: Arc<LsmBTreeHybrid>,
    raft: Arc<RaftClusterManager>,
}

impl RecordsEngine {
    pub fn new(storage: Arc<LsmBTreeHybrid>, raft: Arc<RaftClusterManager>) -> Self {
        Self { storage, raft }
    }

    pub fn save_record(
        &self,
        collection: &str,
        record_id: &str,
        record_class: &str,
        components: Value,
        schema: Option<Value>,
    ) {
        let key = format!("rec:{}:{}", collection, record_id);
        let now_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64;

        let doc = json!({
            "_recordClass": record_class,
            "_timestamp": now_ms,
            "_version": 1,
            "_schema": schema.unwrap_or_else(|| json!({})),
            "components": components
        });

        let json_str = doc.to_string();
        self.storage.put(&key, json_str.as_bytes(), now_ms);

        let cmd = format!("PUT {} {}", key, json_str);
        let raft = self.raft.clone();
        tokio::spawn(async move {
            let _ = raft.replicate_command(&cmd).await;
        });
    }

    pub fn get_record(&self, collection: &str, record_id: &str) -> Option<Value> {
        let key = format!("rec:{}:{}", collection, record_id);
        if let Some(bytes) = self.storage.get(&key) {
            return serde_json::from_slice(&bytes).ok();
        }
        None
    }

    pub fn project_fields(
        &self,
        collection: &str,
        record_id: &str,
        fields: &[String],
    ) -> Option<Value> {
        let full = self.get_record(collection, record_id)?;
        if fields.is_empty() {
            return Some(full);
        }

        let components = full.get("components")?;
        if let Value::Object(comp_map) = components {
            let mut projected = Map::new();
            for f in fields {
                if let Some(v) = comp_map.get(f) {
                    projected.insert(f.clone(), v.clone());
                }
            }
            return Some(Value::Object(projected));
        }

        Some(full)
    }

    pub fn delete_record(&self, collection: &str, record_id: &str) {
        let key = format!("rec:{}:{}", collection, record_id);
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
        let prefix = format!("rec:{}:", collection);
        let raw = self.storage.scan_prefix(&prefix);
        let mut recs = HashMap::new();
        for (k, v) in raw {
            let rid = k[prefix.len()..].to_string();
            if let Ok(val) = serde_json::from_slice(&v) {
                recs.insert(rid, val);
            }
        }
        recs
    }
}

