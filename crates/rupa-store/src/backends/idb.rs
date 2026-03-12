use crate::store::Store;
use rupa_base::Error;

/// A placeholder for IndexedDB-based storage (Browser).
/// Full implementation belongs in Tier 3 adapters.
pub struct IndexedDbStore;

impl Store for IndexedDbStore {
    fn read(&self, _key: &str) -> Result<Option<Vec<u8>>, Error> {
        Err(Error::Unsupported("IndexedDbStore not yet implemented".into()))
    }
    fn write(&self, _key: &str, _value: &[u8]) -> Result<(), Error> {
        Err(Error::Unsupported("IndexedDbStore not yet implemented".into()))
    }
    fn delete(&self, _key: &str) -> Result<(), Error> {
        Err(Error::Unsupported("IndexedDbStore not yet implemented".into()))
    }
}
