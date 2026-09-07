use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::cluster::RaftClusterManager;
use crate::storage::LsmBTreeHybrid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorSearchResult {
    pub id: String,
    pub similarity: f32,
    pub document: Option<Value>,
}

pub struct VectorEngine {
    storage: Arc<LsmBTreeHybrid>,
    raft: Arc<RaftClusterManager>,
    vector_index: RwLock<HashMap<String, Vec<f32>>>,
}

impl VectorEngine {
    pub fn new(storage: Arc<LsmBTreeHybrid>, raft: Arc<RaftClusterManager>) -> Self {
        Self {
            storage,
            raft,
            vector_index: RwLock::new(HashMap::new()),
        }
    }

    pub fn insert_vector(
        &self,
        collection: &str,
        vector_id: &str,
        vector: Vec<f32>,
        metadata: Option<Value>,
    ) {
        let key = format!("vec:{}:{}", collection, vector_id);
        let doc = json!({
            "vector": vector,
            "metadata": metadata.unwrap_or(json!({}))
        });

        let json_str = doc.to_string();
        let now_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64;

        self.storage.put(&key, json_str.as_bytes(), now_ms);
        self.vector_index.write().insert(key.clone(), vector);

        let cmd = format!("PUT {} {}", key, json_str);
        let raft = self.raft.clone();
        tokio::spawn(async move {
            let _ = raft.replicate_command(&cmd).await;
        });
    }

    pub fn get_vector(&self, collection: &str, vector_id: &str) -> Option<Value> {
        let key = format!("vec:{}:{}", collection, vector_id);
        if let Some(bytes) = self.storage.get(&key) {
            return serde_json::from_slice(&bytes).ok();
        }
        None
    }

    pub fn delete_vector(&self, collection: &str, vector_id: &str) {
        let key = format!("vec:{}:{}", collection, vector_id);
        let now_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64;

        self.storage.delete(&key, now_ms);
        self.vector_index.write().remove(&key);

        let cmd = format!("DELETE {}", key);
        let raft = self.raft.clone();
        tokio::spawn(async move {
            let _ = raft.replicate_command(&cmd).await;
        });
    }

    pub fn search_vector(&self, collection: &str, query: &[f32], top_k: usize) -> Vec<VectorSearchResult> {
        let prefix = format!("vec:{}:", collection);
        let mut candidates = Vec::new();

        {
            let idx = self.vector_index.read();
            for (key, vec) in idx.iter() {
                if key.starts_with(&prefix) {
                    let sim = Self::cosine_similarity(query, vec);
                    let id = key[prefix.len()..].to_string();
                    candidates.push((id, sim, key.clone()));
                }
            }
        }

        candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        candidates.truncate(top_k);

        let mut results = Vec::new();
        for (id, similarity, key) in candidates {
            let doc = self.storage.get(&key).and_then(|b| serde_json::from_slice(&b).ok());
            results.push(VectorSearchResult {
                id,
                similarity,
                document: doc,
            });
        }

        results
    }

    pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() || a.is_empty() {
            return 0.0;
        }
        let mut dot = 0.0f32;
        let mut norm_a = 0.0f32;
        let mut norm_b = 0.0f32;

        for i in 0..a.len() {
            dot += a[i] * b[i];
            norm_a += a[i] * a[i];
            norm_b += b[i] * b[i];
        }

        if norm_a == 0.0 || norm_b == 0.0 {
            return 0.0;
        }

        dot / (norm_a.sqrt() * norm_b.sqrt())
    }
}

