//! # Rupa i18n 🌐
//!
//! Internationalization and Localization for the Rupa Framework. 
//! Provides reactive translation and locale management through a 
//! decoupled Port system.

pub mod provider;
pub mod dictionary;
pub mod locale;

pub use provider::{Provider, Translator};
pub use locale::Locale;
use rupa_context::use_context;
use rupa_signals::Memo;
use std::sync::Arc;

/// Translates a key into the current language reactively.
/// 
/// Returns a `Memo<String>` that automatically updates whenever 
/// the global `Locale` or the active `Provider` changes.
///
/// # Examples
///
/// ```
/// use rupa_i18n::translate;
/// let text = translate("welcome.message");
/// ```
pub fn translate(key: impl Into<String>) -> Memo<String> {
    let key = key.into();
    Memo::new(move || {
        if let Some(provider) = use_context::<Provider>() {
            provider.translate(&key)
        } else {
            key.clone()
        }
    })
}

/// A mock implementation of the Translator trait for TDD and headless testing.
pub struct MockTranslator {
    pub language: String,
}

impl Translator for MockTranslator {
    fn translate(&self, key: &str) -> String {
        format!("{}:{}", self.language, key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rupa_context::provide_context;

    #[test]
    fn test_mock_translation_flow() {
        let provider = Provider::new(Locale::En);
        provider.set_translator(Arc::new(MockTranslator { language: "en".into() }));
        
        rupa_context::with_registry(std::sync::Arc::new(rupa_context::Registry::new()), || {
            provide_context(provider);
            let text = translate("greeting");
            assert_eq!(text.get(), "en:greeting");
        });
    }
}
