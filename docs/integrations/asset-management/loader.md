# Module: Resource Loader (`crates/rupa-assets/src/loader.rs`) 🚚

This module provides the asynchronous pipeline for fetching and decoding raw asset bytes.

---

## 🏗️ Trait: `Loader`
Defines how to retrieve data from different sources.

- **`FsLoader`**: Retrieves assets from the local disk.
- **`HttpLoader`**: Fetches assets over the network.
- **`EmbeddedLoader`**: Accesses assets compiled directly into the binary.
