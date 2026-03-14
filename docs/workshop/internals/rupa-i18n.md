# `rupa-i18n` 🌐

**The Globalization Port.** This crate provides **Atoms** for reactive internationalization and localization. It ensures that Rupa applications can be translated into any language with zero UI flicker.

---

## 🏛️ Architectural Role
- **Tier**: Tier 1 (Atoms)
- **Identity**: The Materials & Tools (Ports & Invariants)
- **3S Compliance**: 
    - **Secure (S1)**: Strict key validation prevents missing-translation errors from crashing the UI.
    - **Sustain (S2)**: Reactive `translate` helper automatically triggers UI updates on locale change.
    - **Scalable (S3)**: Supports external dictionary loading for large-scale multi-language apps.

## 🛠️ Key Primitives

| Primitive | Purpose | Features |
| :--- | :--- | :--- |
| **`translate(key)`** | Reactive Translation. | Returns a `Memo<String>` synced to the active locale. |
| **`Translator`** | The i18n Port. | Trait for resolving keys into localized content. |
| **`Provider`** | Context Manager. | Manages active locale and translator instance. |
| **`MockTranslator`** | Testing Backend. | Returns deterministic "key-formatted" strings for tests. |

## 🚀 Usage

```rust
use rupa_i18n::{translate, Locale, Provider};

// 1. Create a reactive translation
let greeting = translate("home.welcome");

// 2. React to changes automatically
Effect::new(move || {
    println!("Visual Text: {}", greeting.get());
});

// 3. Change locale at runtime (e.g., from a settings toggle)
let i18n = use_context::<Provider>().unwrap();
i18n.current_locale.set(Locale::Id); // UI re-renders with Indonesian
```

## 🧪 Testing & Reliability
- **TDD Support**: `MockTranslator` allows tests to assert that components are requesting the correct keys without needing real translation files.
- **Zero Flicker**: Dependency tracking ensures that translations are updated before the next frame is presented.
- **Decoupled**: The core logic doesn't know about JSON or YAML; it only interacts with the `Translator` port.
