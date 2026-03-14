# Module: Store Interface (`crates/rupa-store/src/store.rs`) 💾

This module defines the universal trait for storage backends. It provides the low-level abstraction required to read and write raw bytes to physical media.

---

## 🏗️ Trait: `Store`
The core contract for all persistent drivers.

- `fn read(&self, key: &str) -> Result<Option<Vec<u8>>, Error>`: Retrieves data from the backend.
- `fn write(&self, key: &str, value: &[u8]) -> Result<(), Error>`: Commits data to the backend.
- `fn delete(&self, key: &str) -> Result<(), Error>`: Removes a record from the backend.
