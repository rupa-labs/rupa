use crate::store::Store;
use rupa_base::Error;
use std::fs;
use std::path::PathBuf;

/// A storage backend that saves data as individual files on the disk.
pub struct FileSystemStore {
    root_path: PathBuf,
}

impl FileSystemStore {
    pub fn new(app_name: &str) -> Self {
        let mut path = std::env::current_dir().unwrap_or_default();
        path.push(".rupa_storage");
        path.push(app_name);
        
        if !path.exists() {
            let _ = fs::create_dir_all(&path);
        }
        
        Self { root_path: path }
    }

    fn get_key_path(&self, key: &str) -> PathBuf {
        let mut path = self.root_path.clone();
        path.push(key);
        path
    }
}

impl Store for FileSystemStore {
    fn read(&self, key: &str) -> Result<Option<Vec<u8>>, Error> {
        let path = self.get_key_path(key);
        if !path.exists() {
            return Ok(None);
        }
        
        fs::read(path)
            .map(Some)
            .map_err(|e| Error::Custom(format!("FS Read Error: {}", e)))
    }

    fn write(&self, key: &str, value: &[u8]) -> Result<(), Error> {
        let path = self.get_key_path(key);
        fs::write(path, value)
            .map_err(|e| Error::Custom(format!("FS Write Error: {}", e)))
    }

    fn delete(&self, key: &str) -> Result<(), Error> {
        let path = self.get_key_path(key);
        if path.exists() {
            fs::remove_file(path)
                .map_err(|e| Error::Custom(format!("FS Delete Error: {}", e)))
        } else {
            Ok(())
        }
    }
}
