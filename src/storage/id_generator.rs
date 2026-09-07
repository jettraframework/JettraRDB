use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use parking_lot::RwLock;
use md5::Md5;
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdMode {
    Manual,
    Autoincrement,
    Uuid,
}

impl IdMode {
    pub fn from_str(raw: &str) -> Self {
        match raw.trim().to_uppercase().as_str() {
            "AUTO" | "AUTOINCREMENT" | "AUTO_INCREMENT" | "INCREMENT" => IdMode::Autoincrement,
            "UUID" | "COMPOSITE_UUID" | "COMPOSITE" | "GEN" => IdMode::Uuid,
            _ => IdMode::Manual,
        }
    }
}

pub struct IdGenerator {
    sequences: RwLock<HashMap<String, Arc<AtomicU64>>>,
    cpu_host_signature: String,
}

impl IdGenerator {
    pub fn new() -> Self {
        let sig = Self::compute_cpu_host_signature();
        Self {
            sequences: RwLock::new(HashMap::new()),
            cpu_host_signature: sig,
        }
    }

    pub fn generate_id(&self, collection: &str, mode: IdMode, manual_id: Option<&str>) -> String {
        match mode {
            IdMode::Autoincrement => self.next_sequence_value(collection).to_string(),
            IdMode::Uuid => self.generate_composite_uuid(collection),
            IdMode::Manual => {
                if let Some(id) = manual_id {
                    let trimmed = id.trim();
                    if !trimmed.is_empty() {
                        return trimmed.to_string();
                    }
                }
                self.generate_composite_uuid(collection)
            }
        }
    }

    pub fn next_sequence_value(&self, collection: &str) -> u64 {
        let key = if collection.trim().is_empty() { "default" } else { collection.trim() };
        
        let seq = {
            let r = self.sequences.read();
            r.get(key).cloned()
        };

        let counter = match seq {
            Some(c) => c,
            None => {
                let mut w = self.sequences.write();
                w.entry(key.to_string())
                    .or_insert_with(|| Arc::new(AtomicU64::new(0)))
                    .clone()
            }
        };

        counter.fetch_add(1, Ordering::SeqCst) + 1
    }

    pub fn initialize_sequence(&self, collection: &str, highest_existing_id: u64) {
        let key = if collection.trim().is_empty() { "default" } else { collection.trim() };
        let mut w = self.sequences.write();
        let counter = w.entry(key.to_string()).or_insert_with(|| Arc::new(AtomicU64::new(0)));
        let mut current = counter.load(Ordering::SeqCst);
        while highest_existing_id > current {
            match counter.compare_exchange_weak(current, highest_existing_id, Ordering::SeqCst, Ordering::SeqCst) {
                Ok(_) => break,
                Err(actual) => current = actual,
            }
        }
    }

    pub fn generate_composite_uuid(&self, collection: &str) -> String {
        let now_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        let time_hex = format!("{:012x}", now_ms);
        let coll_hex = Self::compute_collection_hash(collection);
        let random_suffix = Uuid::new_v4().simple().to_string()[..12].to_string();

        format!("{}-{}-{}-{}", self.cpu_host_signature, time_hex, coll_hex, random_suffix)
    }

    fn compute_cpu_host_signature() -> String {
        let mut hasher = Sha256::new();
        let hostname = std::env::var("HOSTNAME").unwrap_or_else(|_| "localhost".to_string());
        let arch = std::env::consts::ARCH;
        let os = std::env::consts::OS;
        hasher.update(format!("{}:{}:{}", hostname, arch, os).as_bytes());
        let digest = hasher.finalize();
        hex::encode(&digest[..4])
    }

    fn compute_collection_hash(collection: &str) -> String {
        if collection.trim().is_empty() {
            return "0000".to_string();
        }
        let mut hasher = Md5::new();
        hasher.update(collection.as_bytes());
        let result = hasher.finalize();
        hex::encode(&result[..2])
    }
}

impl Default for IdGenerator {
    fn default() -> Self {
        Self::new()
    }
}

