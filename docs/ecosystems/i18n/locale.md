# Module: Locale System (`crates/rupa-i18n/src/locale.rs`) 🌐

This module defines the data types representing cultural and regional settings.

---

## 🏗️ `struct Locale`
A robust representation of a language and region (e.g., `en-US`, `id-ID`).

- **Formatting**: Provides methods for culturally aware number, date, and currency formatting.
- **Fallback Logic**: Resolves to a primary language (e.g., `en`) if a regional dialect is unavailable.
