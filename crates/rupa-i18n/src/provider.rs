use rupa_signals::Signal;
use crate::locale::Locale;
use std::sync::Arc;

/// The primary Port for resolving translation keys.
pub trait Translator: Send + Sync {
    fn translate(&self, key: &str, locale: &Locale) -> String;
}

/// The context provider for internationalization.
pub struct Provider {
    pub current_locale: Signal<Locale>,
    pub translator: Arc<dyn Translator>,
}

impl Provider {
    pub fn new(initial: Locale, translator: Arc<dyn Translator>) -> Self {
        Self {
            current_locale: Signal::new(initial),
            translator,
        }
    }

    pub fn translate(&self, key: &str) -> String {
        self.translator.translate(key, &self.current_locale.get())
    }
}
