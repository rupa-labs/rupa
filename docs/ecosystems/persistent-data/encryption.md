# Module: Encryption Layer (`crates/rupa-store/src/encryption.rs`) 🔐

This module handles the security aspect of data persistence, ensuring sensitive information is protected on disk.

---

## 🛡️ Security Features
- **Middleware Integration**: Intercepts `Store::write` calls to encrypt bytes before they reach the driver.
- **Algorithm Standard**: Uses industry-standard authenticated encryption (e.g., AES-GCM).
- **Key Management**: Interfaces with system keychains (via `rupa-support`) to securely store encryption keys.
