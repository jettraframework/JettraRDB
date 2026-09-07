use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use parking_lot::Mutex;

pub struct FileManager {
    file_path: PathBuf,
    file: Mutex<File>,
}

impl FileManager {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path_buf = path.as_ref().to_path_buf();
        if let Some(parent) = path_buf.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path_buf)?;

        Ok(Self {
            file_path: path_buf,
            file: Mutex::new(file),
        })
    }

    pub fn append(&self, data: &[u8]) -> io::Result<u64> {
        let mut f = self.file.lock();
        let offset = f.seek(SeekFrom::End(0))?;
        
        let len = data.len() as u32;
        f.write_all(&len.to_be_bytes())?;
        if len > 0 {
            f.write_all(data)?;
        }
        Ok(offset)
    }

    pub fn read_record(&self, offset: u64) -> io::Result<Vec<u8>> {
        let mut f = self.file.lock();
        f.seek(SeekFrom::Start(offset))?;
        
        let mut len_buf = [0u8; 4];
        f.read_exact(&mut len_buf)?;
        let len = u32::from_be_bytes(len_buf) as usize;
        
        if len == 0 {
            return Ok(Vec::new());
        }
        
        let mut buffer = vec![0u8; len];
        f.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    pub fn sync(&self) -> io::Result<()> {
        let f = self.file.lock();
        f.sync_all()
    }

    pub fn path(&self) -> &Path {
        &self.file_path
    }
}

