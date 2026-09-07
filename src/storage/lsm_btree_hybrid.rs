use std::collections::{BTreeMap, HashMap};
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Local};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

use super::file_manager::FileManager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordVersion {
    pub version_number: usize,
    pub timestamp: i64,
    pub formatted_date: String,
    pub payload: String,
    pub is_current: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseRecordItem {
    pub key: String,
    pub engine: String,
    pub namespace: String,
    pub id: String,
    pub payload: String,
    pub version_count: usize,
    pub timestamp: i64,
}

pub struct DatabasePartition {
    db_name: String,
    db_directory: PathBuf,
    journal_file: PathBuf,
    mem_table: RwLock<BTreeMap<String, Vec<u8>>>,
    disk_index: RwLock<HashMap<String, u64>>,
    version_history: RwLock<HashMap<String, BTreeMap<i64, Vec<u8>>>>,
    file_manager: Arc<FileManager>,
    flush_threshold: usize,
}

impl DatabasePartition {
    pub fn new(root_dir: &Path, db_name: &str) -> Self {
        let clean_name = if db_name.trim().is_empty() || db_name.eq_ignore_ascii_case("_system") {
            "_system".to_string()
        } else {
            db_name.trim().to_string()
        };

        let db_directory = if clean_name == "_system" {
            root_dir.join("system")
        } else {
            root_dir.join("databases").join(&clean_name)
        };

        let _ = fs::create_dir_all(&db_directory);
        let journal_file = db_directory.join("wal.jettra");
        let data_file = db_directory.join("data_0.jettra");
        let file_manager = Arc::new(FileManager::open(&data_file).expect("Failed to open data file"));

        let partition = Self {
            db_name: clean_name,
            db_directory,
            journal_file,
            mem_table: RwLock::new(BTreeMap::new()),
            disk_index: RwLock::new(HashMap::new()),
            version_history: RwLock::new(HashMap::new()),
            file_manager,
            flush_threshold: 1000,
        };

        partition.load_from_wal();
        partition
    }

    pub fn get_db_name(&self) -> &str {
        &self.db_name
    }

    pub fn get_db_directory(&self) -> &Path {
        &self.db_directory
    }

    fn load_from_wal(&self) {
        if !self.journal_file.exists() {
            return;
        }

        let file = match File::open(&self.journal_file) {
            Ok(f) => f,
            Err(_) => return,
        };
        let mut reader = BufReader::new(file);

        let mut mem = self.mem_table.write();
        let mut hist = self.version_history.write();

        loop {
            // Read key length
            let mut key_len_buf = [0u8; 2];
            if reader.read_exact(&mut key_len_buf).is_err() {
                break; // EOF
            }
            let key_len = u16::from_be_bytes(key_len_buf) as usize;

            let mut key_buf = vec![0u8; key_len];
            if reader.read_exact(&mut key_buf).is_err() {
                break;
            }
            let key = match String::from_utf8(key_buf) {
                Ok(s) => s,
                Err(_) => break,
            };

            // Read timestamp
            let mut ts_buf = [0u8; 8];
            if reader.read_exact(&mut ts_buf).is_err() {
                break;
            }
            let ts = i64::from_be_bytes(ts_buf);

            // Read data length
            let mut data_len_buf = [0u8; 4];
            if reader.read_exact(&mut data_len_buf).is_err() {
                break;
            }
            let data_len = u32::from_be_bytes(data_len_buf) as usize;

            if data_len > 0 {
                let mut data = vec![0u8; data_len];
                if reader.read_exact(&mut data).is_err() {
                    break;
                }
                let versioned_key = format!("{}@{}", key, ts);
                mem.insert(versioned_key, data.clone());
                hist.entry(key).or_default().insert(ts, data);
            } else {
                // Tombstone deletion
                let prefix = format!("{}@", key);
                let to_remove: Vec<String> = mem
                    .keys()
                    .filter(|k| k.as_str() == key || k.starts_with(&prefix))
                    .cloned()
                    .collect();
                for k in to_remove {
                    mem.remove(&k);
                }
                hist.remove(&key);
                mem.insert(format!("{}@{}", key, ts), Vec::new());
            }
        }
    }

    fn append_wal(&self, key: &str, ts: i64, data: &[u8]) {
        let _ = fs::create_dir_all(&self.db_directory);
        if let Ok(file) = OpenOptions::new().create(true).append(true).open(&self.journal_file) {
            let mut writer = BufWriter::new(file);
            let key_bytes = key.as_bytes();
            let key_len = key_bytes.len() as u16;
            let _ = writer.write_all(&key_len.to_be_bytes());
            let _ = writer.write_all(key_bytes);
            let _ = writer.write_all(&ts.to_be_bytes());
            let data_len = data.len() as u32;
            let _ = writer.write_all(&data_len.to_be_bytes());
            if !data.is_empty() {
                let _ = writer.write_all(data);
            }
            let _ = writer.flush();
        }
    }

    pub fn put(&self, key: &str, data: &[u8], timestamp: i64) {
        if key.is_empty() {
            return;
        }

        let effective_ts = {
            let mut hist = self.version_history.write();
            let map = hist.entry(key.to_string()).or_default();
            let mut ts = timestamp;
            if let Some((&last_ts, _)) = map.iter().next_back() {
                if last_ts >= ts {
                    ts = last_ts + 1;
                }
            }
            map.insert(ts, data.to_vec());
            ts
        };

        let versioned_key = format!("{}@{}", key, effective_ts);
        {
            let mut mem = self.mem_table.write();
            mem.insert(versioned_key, data.to_vec());
        }

        self.append_wal(key, effective_ts, data);

        if self.mem_table.read().len() >= self.flush_threshold {
            self.flush_to_btree();
        }
    }

    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        // 1. Check version history in-memory
        {
            let hist = self.version_history.read();
            if let Some(map) = hist.get(key) {
                if let Some((_, val)) = map.iter().next_back() {
                    if !val.is_empty() {
                        return Some(val.clone());
                    }
                    return None; // Tombstone
                }
            }
        }

        // 2. Check MemTable
        {
            let mem = self.mem_table.read();
            let upper_bound = format!("{}@{}", key, i64::MAX);
            if let Some((k, v)) = mem.range(..=upper_bound).next_back() {
                let prefix = format!("{}@", key);
                if k.starts_with(&prefix) || k == key {
                    if !v.is_empty() {
                        return Some(v.clone());
                    }
                    return None;
                }
            }
        }

        // 3. Check Disk Index
        let offset = {
            let idx = self.disk_index.read();
            idx.get(key).copied()
        };

        if let Some(off) = offset {
            if let Ok(record) = self.file_manager.read_record(off) {
                if !record.is_empty() {
                    return Some(record);
                }
            }
        }

        None
    }

    pub fn delete(&self, key: &str, timestamp: i64) {
        let prefix = format!("{}@", key);
        {
            let mut mem = self.mem_table.write();
            let to_remove: Vec<String> = mem
                .keys()
                .filter(|k| k.as_str() == key || k.starts_with(&prefix))
                .cloned()
                .collect();
            for k in to_remove {
                mem.remove(&k);
            }
            mem.insert(format!("{}@{}", key, timestamp), Vec::new());
        }

        {
            let mut hist = self.version_history.write();
            hist.remove(key);
        }

        {
            let mut idx = self.disk_index.write();
            idx.remove(key);
        }

        self.append_wal(key, timestamp, &[]);
    }

    pub fn scan_prefix(&self, prefix: &str) -> HashMap<String, Vec<u8>> {
        let mut results = HashMap::new();

        // Check MemTable
        {
            let mem = self.mem_table.read();
            for (k, _) in mem.iter() {
                if k.starts_with(prefix) {
                    let base_key = if let Some(idx) = k.rfind('@') {
                        &k[..idx]
                    } else {
                        k.as_str()
                    };
                    if !results.contains_key(base_key) {
                        if let Some(val) = self.get(base_key) {
                            results.insert(base_key.to_string(), val);
                        }
                    }
                }
            }
        }

        // Check Disk Index
        {
            let idx = self.disk_index.read();
            for k in idx.keys() {
                if k.starts_with(prefix) && !results.contains_key(k) {
                    if let Some(val) = self.get(k) {
                        results.insert(k.clone(), val);
                    }
                }
            }
        }

        results
    }

    pub fn get_version_history(&self, key: &str) -> Vec<RecordVersion> {
        let hist_map = {
            let hist = self.version_history.read();
            hist.get(key).cloned()
        };

        let mut time_map = hist_map.unwrap_or_default();

        if time_map.is_empty() {
            let prefix = format!("{}@", key);
            let mem = self.mem_table.read();
            for (k, v) in mem.iter() {
                if k.starts_with(&prefix) {
                    if let Ok(ts) = k[prefix.len()..].parse::<i64>() {
                        if !v.is_empty() {
                            time_map.insert(ts, v.clone());
                        }
                    }
                }
            }
        }

        if time_map.is_empty() {
            if let Some(val) = self.get(key) {
                let now_ms = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as i64;
                time_map.insert(now_ms, val);
            }
        }

        if time_map.is_empty() {
            return Vec::new();
        }

        let latest_ts = *time_map.keys().next_back().unwrap_or(&0);
        let mut counter = 1;

        let mut chronological = Vec::new();
        for (&ts, data) in &time_map {
            let payload = String::from_utf8_lossy(data).to_string();
            let naive = DateTime::from_timestamp_millis(ts).unwrap_or_default();
            let formatted_date = naive.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string();
            let is_current = ts == latest_ts;

            chronological.push(RecordVersion {
                version_number: counter,
                timestamp: ts,
                formatted_date,
                payload,
                is_current,
            });
            counter += 1;
        }

        chronological.reverse();
        chronological
    }

    pub fn get_version_count(&self, key: &str) -> usize {
        let hist = self.version_history.read();
        if let Some(m) = hist.get(key) {
            return m.len().max(1);
        }
        if self.get(key).is_some() { 1 } else { 0 }
    }

    pub fn get_version(&self, key: &str, timestamp: i64) -> Option<Vec<u8>> {
        let hist = self.version_history.read();
        if let Some(m) = hist.get(key) {
            if let Some(data) = m.get(&timestamp) {
                return Some(data.clone());
            }
        }
        let versioned_key = format!("{}@{}", key, timestamp);
        let mem = self.mem_table.read();
        mem.get(&versioned_key).cloned()
    }

    pub fn restore_version(&self, key: &str, timestamp: i64) -> bool {
        if let Some(data) = self.get_version(key, timestamp) {
            if !data.is_empty() {
                let now_ms = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as i64;
                self.put(key, &data, now_ms);
                return true;
            }
        }
        false
    }

    pub fn flush_to_btree(&self) {
        let entries: Vec<(String, Vec<u8>)> = {
            let mut mem = self.mem_table.write();
            if mem.is_empty() {
                return;
            }
            let drained = mem.clone();
            mem.clear();
            drained.into_iter().collect()
        };

        let mut idx = self.disk_index.write();
        for (k, v) in entries {
            if v.is_empty() {
                continue; // Skip tombstone in SSTable
            }
            if let Ok(offset) = self.file_manager.append(&v) {
                let base_key = if let Some(pos) = k.rfind('@') {
                    k[..pos].to_string()
                } else {
                    k
                };
                idx.insert(base_key, offset);
            }
        }
        let _ = self.file_manager.sync();
    }

    pub fn drop_partition(&self) {
        self.mem_table.write().clear();
        self.disk_index.write().clear();
        self.version_history.write().clear();
        let _ = fs::remove_dir_all(&self.db_directory);
    }

    pub fn close(&self) {
        self.flush_to_btree();
    }
}

pub struct LsmBTreeHybrid {
    storage_directory: PathBuf,
    partitions: RwLock<HashMap<String, Arc<DatabasePartition>>>,
}

impl LsmBTreeHybrid {
    pub fn new<P: AsRef<Path>>(storage_directory: P) -> Self {
        let dir = storage_directory.as_ref().to_path_buf();
        let _ = fs::create_dir_all(&dir);

        let hybrid = Self {
            storage_directory: dir.clone(),
            partitions: RwLock::new(HashMap::new()),
        };

        // Initialize system partition
        hybrid.get_partition("_system");

        // Scan existing databases under databases/
        let db_root = dir.join("databases");
        if db_root.is_dir() {
            if let Ok(entries) = fs::read_dir(db_root) {
                for entry in entries.flatten() {
                    if entry.path().is_dir() {
                        if let Some(db_name) = entry.file_name().to_str() {
                            hybrid.get_partition(db_name);
                        }
                    }
                }
            }
        }

        hybrid
    }

    pub fn extract_database_from_key(key: &str) -> String {
        let trimmed = key.trim();
        if trimmed.is_empty() || trimmed.starts_with("sys:") || trimmed.starts_with("_system:") || trimmed.starts_with("system:") {
            return "_system".to_string();
        }

        if let Some(first_colon) = trimmed.find(':') {
            let pfx = trimmed[..first_colon].to_lowercase();
            match pfx.as_str() {
                "rec" | "doc" | "geo" | "vec" | "obj" | "kv" | "ts" | "graph" | "col" | "schema" | "idx" => {
                    let rest = &trimmed[first_colon + 1..];
                    if rest.is_empty() {
                        return "_system".to_string();
                    }
                    if let Some(next_colon) = rest.find(':') {
                        let cand = &rest[..next_colon];
                        if cand.is_empty() { "_system".to_string() } else { cand.to_string() }
                    } else {
                        rest.to_string()
                    }
                }
                _ => trimmed[..first_colon].to_string(),
            }
        } else {
            "_system".to_string()
        }
    }

    pub fn get_partition(&self, db_name: &str) -> Arc<DatabasePartition> {
        let clean = if db_name.trim().is_empty() { "_system" } else { db_name.trim() };

        {
            let r = self.partitions.read();
            if let Some(p) = r.get(clean) {
                return p.clone();
            }
            for (k, v) in r.iter() {
                if k.eq_ignore_ascii_case(clean) {
                    return v.clone();
                }
            }
        }

        let mut w = self.partitions.write();
        let partition = Arc::new(DatabasePartition::new(&self.storage_directory, clean));
        w.insert(clean.to_string(), partition.clone());
        partition
    }

    pub fn get_database_names(&self) -> Vec<String> {
        let mut dbs = Vec::new();
        let r = self.partitions.read();
        for k in r.keys() {
            if k != "_system" {
                dbs.push(k.clone());
            }
        }
        let db_root = self.storage_directory.join("databases");
        if db_root.is_dir() {
            if let Ok(entries) = fs::read_dir(db_root) {
                for entry in entries.flatten() {
                    if entry.path().is_dir() {
                        if let Some(name) = entry.file_name().to_str() {
                            if !dbs.contains(&name.to_string()) && name != "_system" {
                                dbs.push(name.to_string());
                            }
                        }
                    }
                }
            }
        }
        dbs.sort();
        dbs
    }

    pub fn drop_database(&self, db_name: &str) {
        if db_name.trim().is_empty() || db_name.eq_ignore_ascii_case("_system") {
            return;
        }
        let part = {
            let mut w = self.partitions.write();
            w.remove(db_name.trim())
        };
        if let Some(p) = part {
            p.drop_partition();
        } else {
            let dir = self.storage_directory.join("databases").join(db_name.trim());
            let _ = fs::remove_dir_all(dir);
        }
    }

    pub fn create_database(&self, db_name: &str) -> Result<(), String> {
        let clean = db_name.trim();
        if clean.is_empty() {
            return Err("Database name cannot be empty".to_string());
        }
        if clean.eq_ignore_ascii_case("_system") {
            return Err("Cannot explicitly create _system database".to_string());
        }
        if !clean.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            return Err("Database name can only contain alphanumeric characters, hyphens, and underscores".to_string());
        }
        let existing = self.get_database_names();
        if existing.iter().any(|d| d.eq_ignore_ascii_case(clean)) {
            return Err(format!("Database '{}' already exists", clean));
        }

        self.get_partition(clean);
        Ok(())
    }

    pub fn rename_database(&self, old_name: &str, new_name: &str) -> Result<(), String> {
        let old_clean = old_name.trim();
        let new_clean = new_name.trim();

        if old_clean.is_empty() || new_clean.is_empty() {
            return Err("Database names cannot be empty".to_string());
        }
        if old_clean.eq_ignore_ascii_case("_system") || new_clean.eq_ignore_ascii_case("_system") {
            return Err("Cannot rename to or from _system database".to_string());
        }
        if old_clean.eq_ignore_ascii_case(new_clean) {
            return Err("Old name and new name are identical".to_string());
        }
        if !new_clean.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            return Err("New database name can only contain alphanumeric characters, hyphens, and underscores".to_string());
        }

        let existing = self.get_database_names();
        if !existing.iter().any(|d| d.eq_ignore_ascii_case(old_clean)) {
            return Err(format!("Database '{}' not found", old_clean));
        }
        if existing.iter().any(|d| d.eq_ignore_ascii_case(new_clean)) {
            return Err(format!("Target database '{}' already exists", new_clean));
        }

        // 1. Close and flush old partition
        {
            let mut w = self.partitions.write();
            if let Some(old_part) = w.remove(old_clean) {
                old_part.close();
            }
        }

        // 2. Rename on disk
        let old_dir = self.storage_directory.join("databases").join(old_clean);
        let new_dir = self.storage_directory.join("databases").join(new_clean);

        if old_dir.exists() {
            if let Err(e) = fs::rename(&old_dir, &new_dir) {
                return Err(format!("Failed to rename database directory: {}", e));
            }
        } else {
            let _ = fs::create_dir_all(&new_dir);
        }

        // 3. Initialize new partition
        self.get_partition(new_clean);
        Ok(())
    }

    pub fn get_database_records(&self, db_name: &str) -> Vec<DatabaseRecordItem> {
        let mut items = Vec::new();
        let part = self.get_partition(db_name);
        let raw_entries = part.scan_prefix("");

        for (key, val) in raw_entries {
            let (engine, namespace, id) = Self::parse_key_metadata(&key, db_name);
            let payload = String::from_utf8_lossy(&val).to_string();
            let version_count = part.get_version_count(&key);
            let timestamp = part
                .get_version_history(&key)
                .first()
                .map(|v| v.timestamp)
                .unwrap_or(0);

            items.push(DatabaseRecordItem {
                key,
                engine,
                namespace,
                id,
                payload,
                version_count,
                timestamp,
            });
        }

        items.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        items
    }

    pub fn parse_key_metadata(key: &str, default_ns: &str) -> (String, String, String) {
        let parts: Vec<&str> = key.split(':').collect();
        if parts.is_empty() {
            return ("DOCUMENT".to_string(), default_ns.to_string(), key.to_string());
        }

        match parts[0].to_lowercase().as_str() {
            "doc" => {
                let ns = parts.get(1).copied().unwrap_or(default_ns);
                let id = parts.get(2..).map(|s| s.join(":")).unwrap_or_else(|| "unknown".to_string());
                ("DOCUMENT".to_string(), ns.to_string(), id)
            }
            "vec" => {
                let ns = parts.get(1).copied().unwrap_or(default_ns);
                let id = parts.get(2..).map(|s| s.join(":")).unwrap_or_else(|| "unknown".to_string());
                ("VECTOR".to_string(), ns.to_string(), id)
            }
            "graph" => {
                let ns = parts.get(1).copied().unwrap_or(default_ns);
                let id = parts.get(2..).map(|s| s.join(":")).unwrap_or_else(|| "unknown".to_string());
                ("GRAPH".to_string(), ns.to_string(), id)
            }
            "ts" => {
                let ns = parts.get(1).copied().unwrap_or(default_ns);
                let id = parts.get(2..).map(|s| s.join(":")).unwrap_or_else(|| "unknown".to_string());
                ("TIMESERIES".to_string(), ns.to_string(), id)
            }
            "col" => {
                let ns = parts.get(1).copied().unwrap_or(default_ns);
                let id = parts.get(2..).map(|s| s.join(":")).unwrap_or_else(|| "unknown".to_string());
                ("COLUMN".to_string(), ns.to_string(), id)
            }
            "kv" => {
                let ns = parts.get(1).copied().unwrap_or(default_ns);
                let id = parts.get(2..).map(|s| s.join(":")).unwrap_or_else(|| "unknown".to_string());
                ("KEYVALUE".to_string(), ns.to_string(), id)
            }
            "geo" => {
                let ns = parts.get(1).copied().unwrap_or(default_ns);
                let id = parts.get(2..).map(|s| s.join(":")).unwrap_or_else(|| "unknown".to_string());
                ("GEOSPATIAL".to_string(), ns.to_string(), id)
            }
            "obj" => {
                let ns = parts.get(1).copied().unwrap_or(default_ns);
                let id = parts.get(2..).map(|s| s.join(":")).unwrap_or_else(|| "unknown".to_string());
                ("OBJECT".to_string(), ns.to_string(), id)
            }
            "rec" => {
                let ns = parts.get(1).copied().unwrap_or(default_ns);
                let id = parts.get(2..).map(|s| s.join(":")).unwrap_or_else(|| "unknown".to_string());
                ("RECORDS".to_string(), ns.to_string(), id)
            }
            _ => {
                if parts.len() >= 2 {
                    ("DOCUMENT".to_string(), parts[0].to_string(), parts[1..].join(":"))
                } else {
                    ("DOCUMENT".to_string(), default_ns.to_string(), key.to_string())
                }
            }
        }
    }

    pub fn put(&self, key: &str, data: &[u8], timestamp: i64) {
        let db = Self::extract_database_from_key(key);
        self.get_partition(&db).put(key, data, timestamp);
    }

    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        let db = Self::extract_database_from_key(key);
        let val = self.get_partition(&db).get(key);
        if val.is_none() && db != "_system" {
            return self.get_partition("_system").get(key);
        }
        val
    }

    pub fn delete(&self, key: &str, timestamp: i64) {
        let db = Self::extract_database_from_key(key);
        self.get_partition(&db).delete(key, timestamp);
    }

    pub fn scan_prefix(&self, prefix: &str) -> HashMap<String, Vec<u8>> {
        let mut results = HashMap::new();
        if prefix.is_empty() {
            let parts: Vec<Arc<DatabasePartition>> = self.partitions.read().values().cloned().collect();
            for p in parts {
                results.extend(p.scan_prefix(""));
            }
            return results;
        }

        let db = Self::extract_database_from_key(prefix);
        if db != "_system" {
            results.extend(self.get_partition(&db).scan_prefix(prefix));
        } else {
            let parts: Vec<Arc<DatabasePartition>> = self.partitions.read().values().cloned().collect();
            for p in parts {
                results.extend(p.scan_prefix(prefix));
            }
        }
        results
    }

    pub fn get_version_history(&self, key: &str) -> Vec<RecordVersion> {
        let db = Self::extract_database_from_key(key);
        self.get_partition(&db).get_version_history(key)
    }

    pub fn get_version_count(&self, key: &str) -> usize {
        let db = Self::extract_database_from_key(key);
        self.get_partition(&db).get_version_count(key)
    }

    pub fn get_version(&self, key: &str, timestamp: i64) -> Option<Vec<u8>> {
        let db = Self::extract_database_from_key(key);
        self.get_partition(&db).get_version(key, timestamp)
    }

    pub fn restore_version(&self, key: &str, timestamp: i64) -> bool {
        let db = Self::extract_database_from_key(key);
        self.get_partition(&db).restore_version(key, timestamp)
    }

    pub fn close(&self) {
        let parts: Vec<Arc<DatabasePartition>> = self.partitions.read().values().cloned().collect();
        for p in parts {
            p.close();
        }
    }
}

