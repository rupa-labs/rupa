pub mod provider;
pub mod dictionary;
pub mod locale;

pub use provider::{Provider, Translator};
pub use locale::Locale;
use rupa_context::use_context;
use rupa_signals::Memo;

/// Translates a key into the current language reactively.
/// Returns a Memo<String> that updates automatically when the locale changes.
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
