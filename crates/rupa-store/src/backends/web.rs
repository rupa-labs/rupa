use crate::store::Store;
use rupa_base::Error;

/// A storage backend that uses the Browser's LocalStorage API.
pub struct WebStorageStore;

impl WebStorageStore {
    pub fn new() -> Self { Self }
}

impl Store for WebStorageStore {
    fn read(&self, key: &str) -> Result<Option<Vec<u8>>, Error> {
        #[cfg(target_arch = "wasm32")]
        {
            let window = web_sys::window().ok_or_else(|| Error::Platform("No window found".into()))?;
            let storage = window.local_storage()
                .map_err(|_| Error::Platform("Storage access denied".into()))?
                .ok_or_else(|| Error::Platform("No local storage found".into()))?;
            
            let val = storage.get_item(key).map_err(|_| Error::Platform("Read error".into()))?;
            Ok(val.map(|s| s.into_bytes()))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Err(Error::Platform("WebStorage only available on WASM".into()))
        }
    }

    fn write(&self, key: &str, value: &[u8]) -> Result<(), Error> {
        #[cfg(target_arch = "wasm32")]
        {
            let window = web_sys::window().unwrap();
            let storage = window.local_storage().unwrap().unwrap();
            let val = String::from_utf8_lossy(value);
            storage.set_item(key, &val).map_err(|_| Error::Platform("Write error".into()))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = (key, value);
            Err(Error::Platform("WebStorage only available on WASM".into()))
        }
    }

    fn delete(&self, key: &str) -> Result<(), Error> {
        #[cfg(target_arch = "wasm32")]
        {
            let window = web_sys::window().unwrap();
            let storage = window.local_storage().unwrap().unwrap();
            storage.remove_item(key).map_err(|_| Error::Platform("Delete error".into()))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = key;
            Err(Error::Platform("WebStorage only available on WASM".into()))
        }
    }
}
