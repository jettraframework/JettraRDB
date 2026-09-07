use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::cluster::RaftClusterManager;
use crate::storage::LsmBTreeHybrid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub from: String,
    pub to: String,
    pub label: String,
    pub properties: Value,
}

pub struct GraphEngine {
    storage: Arc<LsmBTreeHybrid>,
    raft: Arc<RaftClusterManager>,
}

impl GraphEngine {
    pub fn new(storage: Arc<LsmBTreeHybrid>, raft: Arc<RaftClusterManager>) -> Self {
        Self { storage, raft }
    }

    pub fn add_node(&self, graph_id: &str, node_id: &str, data: Value) {
        let key = format!("graph:{}:node:{}", graph_id, node_id);
        let json_str = data.to_string();
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

    pub fn get_node(&self, graph_id: &str, node_id: &str) -> Option<Value> {
        let key = format!("graph:{}:node:{}", graph_id, node_id);
        if let Some(bytes) = self.storage.get(&key) {
            return serde_json::from_slice(&bytes).ok();
        }
        None
    }

    pub fn add_edge(
        &self,
        graph_id: &str,
        from_node: &str,
        to_node: &str,
        label: &str,
        properties: Option<Value>,
    ) {
        let key = format!("graph:{}:edge:{}:{}:{}", graph_id, from_node, to_node, label);
        let props = properties.unwrap_or_else(|| serde_json::json!({}));
        let json_str = props.to_string();
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

    pub fn delete_node(&self, graph_id: &str, node_id: &str) {
        let key = format!("graph:{}:node:{}", graph_id, node_id);
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

    pub fn delete_edge(&self, graph_id: &str, from_node: &str, to_node: &str, label: &str) {
        let key = format!("graph:{}:edge:{}:{}:{}", graph_id, from_node, to_node, label);
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

    pub fn list_nodes(&self, graph_id: &str) -> HashMap<String, Value> {
        let prefix = format!("graph:{}:node:", graph_id);
        let raw = self.storage.scan_prefix(&prefix);
        let mut nodes = HashMap::new();
        for (k, v) in raw {
            let nid = k[prefix.len()..].to_string();
            if let Ok(val) = serde_json::from_slice(&v) {
                nodes.insert(nid, val);
            }
        }
        nodes
    }

    pub fn get_outgoing_edges(&self, graph_id: &str, from_node: &str) -> Vec<GraphEdge> {
        let prefix = format!("graph:{}:edge:{}:", graph_id, from_node);
        let raw = self.storage.scan_prefix(&prefix);
        let mut edges = Vec::new();
        for (k, v) in raw {
            let rest = &k[prefix.len()..];
            if let Some(colon) = rest.find(':') {
                let to_node = rest[..colon].to_string();
                let label = rest[colon + 1..].to_string();
                let props = serde_json::from_slice(&v).unwrap_or(serde_json::json!({}));
                edges.push(GraphEdge {
                    from: from_node.to_string(),
                    to: to_node,
                    label,
                    properties: props,
                });
            }
        }
        edges
    }
}

