use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::cluster::RaftClusterManager;
use crate::storage::LsmBTreeHybrid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoLocationResult {
    pub id: String,
    pub lat: f64,
    pub lon: f64,
    pub distance_km: f64,
    pub document: Value,
}

pub struct GeospatialEngine {
    storage: Arc<LsmBTreeHybrid>,
    raft: Arc<RaftClusterManager>,
}

impl GeospatialEngine {
    pub fn new(storage: Arc<LsmBTreeHybrid>, raft: Arc<RaftClusterManager>) -> Self {
        Self { storage, raft }
    }

    pub fn insert_location(
        &self,
        collection: &str,
        loc_id: &str,
        lat: f64,
        lon: f64,
        metadata: Option<Value>,
    ) {
        let key = format!("geo:{}:{}", collection, loc_id);
        let doc = json!({
            "coordinates": {
                "lat": lat,
                "lon": lon
            },
            "metadata": metadata.unwrap_or(json!({}))
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

    pub fn get_location(&self, collection: &str, loc_id: &str) -> Option<Value> {
        let key = format!("geo:{}:{}", collection, loc_id);
        if let Some(bytes) = self.storage.get(&key) {
            return serde_json::from_slice(&bytes).ok();
        }
        None
    }

    pub fn delete_location(&self, collection: &str, loc_id: &str) {
        let key = format!("geo:{}:{}", collection, loc_id);
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
        let prefix = format!("geo:{}:", collection);
        let raw = self.storage.scan_prefix(&prefix);
        let mut locs = HashMap::new();
        for (k, v) in raw {
            let id = k[prefix.len()..].to_string();
            if let Ok(val) = serde_json::from_slice(&v) {
                locs.insert(id, val);
            }
        }
        locs
    }

    pub fn search_radius(
        &self,
        collection: &str,
        center_lat: f64,
        center_lon: f64,
        radius_km: f64,
    ) -> Vec<GeoLocationResult> {
        let all = self.list(collection);
        let mut results = Vec::new();

        for (id, doc) in all {
            if let Some(coords) = doc.get("coordinates") {
                let lat = coords.get("lat").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let lon = coords.get("lon").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let dist = Self::haversine_distance(center_lat, center_lon, lat, lon);
                if dist <= radius_km {
                    results.push(GeoLocationResult {
                        id,
                        lat,
                        lon,
                        distance_km: dist,
                        document: doc,
                    });
                }
            }
        }

        results.sort_by(|a, b| a.distance_km.partial_cmp(&b.distance_km).unwrap_or(std::cmp::Ordering::Equal));
        results
    }

    pub fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
        const EARTH_RADIUS_KM: f64 = 6371.0;
        let d_lat = (lat2 - lat1).to_radians();
        let d_lon = (lon2 - lon1).to_radians();

        let a = (d_lat / 2.0).sin().powi(2)
            + lat1.to_radians().cos() * lat2.to_radians().cos() * (d_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        EARTH_RADIUS_KM * c
    }
}

