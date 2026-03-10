use rupa_support::Error;

/// The core contract for all physical storage backends in Rupa.
pub trait Store: Send + Sync {
    /// Reads raw bytes for a given key.
    fn read(&self, key: &str) -> Result<Option<Vec<u8>>, Error>;
    
    /// Writes raw bytes to a specific key.
    fn write(&self, key: &str, value: &[u8]) -> Result<(), Error>;
    
    /// Removes a key and its associated value from the storage.
    fn delete(&self, key: &str) -> Result<(), Error>;
    
    /// Checks if a key exists in the storage.
    fn exists(&self, key: &str) -> bool {
        self.read(key).map(|v| v.is_some()).unwrap_or(false)
    }
}
