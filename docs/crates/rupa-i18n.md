# `rupa-i18n` 🌍

**The Translation Port.** Reactive internationalization and localization services.

## 🛠️ Key Features

- **`Translator`**: Port for resolving keys into localized strings.
- **`Provider`**: Context-aware manager for active locale and translations.
- **`translate`**: Global reactive helper for UI components.

## 🚀 Usage

```rust
use rupa_i18n::{translate, Locale};

// 1. In UI Component
let greeting = translate("welcome_message");

// 2. Change locale
let i18n = use_context::<Provider>().unwrap();
i18n.current_locale.set(Locale::new("id", "ID")); // UI updates instantly
```
