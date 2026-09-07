use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::{json, Value};

use crate::cluster::RaftClusterManager;
use crate::storage::LsmBTreeHybrid;

pub struct TimeSeriesEngine {
    storage: Arc<LsmBTreeHybrid>,
    raft: Arc<RaftClusterManager>,
}

impl TimeSeriesEngine {
    pub fn new(storage: Arc<LsmBTreeHybrid>, raft: Arc<RaftClusterManager>) -> Self {
        Self { storage, raft }
    }

    pub fn insert(&self, measurement: &str, timestamp: i64, mut data_point: Value) {
        let key = format!("ts:{}:{}", measurement, timestamp);
        if let Value::Object(ref mut map) = data_point {
            map.insert("timestamp".to_string(), json!(timestamp));
        }

        let json_str = data_point.to_string();
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

    pub fn get(&self, measurement: &str, timestamp: i64) -> Option<Value> {
        let key = format!("ts:{}:{}", measurement, timestamp);
        if let Some(bytes) = self.storage.get(&key) {
            return serde_json::from_slice(&bytes).ok();
        }
        None
    }

    pub fn delete(&self, measurement: &str, timestamp: i64) {
        let key = format!("ts:{}:{}", measurement, timestamp);
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

    pub fn list(&self, measurement: &str) -> BTreeMap<i64, Value> {
        let prefix = format!("ts:{}:", measurement);
        let raw = self.storage.scan_prefix(&prefix);
        let mut points = BTreeMap::new();
        for (k, v) in raw {
            if let Ok(ts) = k[prefix.len()..].parse::<i64>() {
                if let Ok(val) = serde_json::from_slice(&v) {
                    points.insert(ts, val);
                }
            }
        }
        points
    }

    pub fn query_range(&self, measurement: &str, from_ts: i64, to_ts: i64) -> Vec<Value> {
        let all = self.list(measurement);
        all.range(from_ts..=to_ts).map(|(_, v)| v.clone()).collect()
    }
}

