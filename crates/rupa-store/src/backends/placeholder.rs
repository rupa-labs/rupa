use crate::store::Store;
use rupa_base::Error;

/// A placeholder for SQLite-based storage.
/// Full implementation belongs in Tier 3 adapters.
pub struct SqliteStore;

impl Store for SqliteStore {
    fn read(&self, _key: &str) -> Result<Option<Vec<u8>>, Error> {
        Err(Error::Unsupported("SqliteStore not yet implemented".into()))
    }
    fn write(&self, _key: &str, _value: &[u8]) -> Result<(), Error> {
        Err(Error::Unsupported("SqliteStore not yet implemented".into()))
    }
    fn delete(&self, _key: &str) -> Result<(), Error> {
        Err(Error::Unsupported("SqliteStore not yet implemented".into()))
    }
}
