use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use chrono::Local;

pub struct BackupManager {
    storage_dir: PathBuf,
}

impl BackupManager {
    pub fn new<P: AsRef<Path>>(storage_dir: P) -> Self {
        Self {
            storage_dir: storage_dir.as_ref().to_path_buf(),
        }
    }

    pub fn create_backup(&self) -> io::Result<PathBuf> {
        let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
        let parent = self.storage_dir.parent().unwrap_or_else(|| Path::new("."));
        let backup_dir = parent.join(format!("jettra_backup_{}", timestamp));

        fs::create_dir_all(&backup_dir)?;
        Self::copy_dir_recursive(&self.storage_dir, &backup_dir)?;

        println!("Backup created successfully at: {:?}", backup_dir);
        Ok(backup_dir)
    }

    pub fn restore_latest_backup(&self) -> bool {
        let parent = match self.storage_dir.parent() {
            Some(p) => p,
            None => {
                eprintln!("Cannot auto-restore: storage directory has no parent.");
                return false;
            }
        };

        let mut backups = Vec::new();
        if let Ok(entries) = fs::read_dir(parent) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir() {
                    if let Some(name) = p.file_name().and_then(|n| n.to_str()) {
                        if name.starts_with("jettra_backup_") {
                            backups.push(p);
                        }
                    }
                }
            }
        }

        backups.sort();
        if let Some(latest) = backups.last() {
            println!("Restoring latest backup from: {:?}", latest);
            let _ = fs::create_dir_all(&self.storage_dir);
            if let Err(e) = Self::copy_dir_recursive(latest, &self.storage_dir) {
                eprintln!("Failed to restore backup: {}", e);
                return false;
            }
            println!("Restore completed successfully.");
            return true;
        }

        println!("No backup directories found to restore.");
        false
    }

    fn copy_dir_recursive(src: &Path, dst: &Path) -> io::Result<()> {
        if !src.exists() {
            return Ok(());
        }
        fs::create_dir_all(dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());
            if src_path.is_dir() {
                Self::copy_dir_recursive(&src_path, &dst_path)?;
            } else {
                fs::copy(&src_path, &dst_path)?;
            }
        }
        Ok(())
    }
}

