use rupa_signals::Signal;
use crate::locale::Locale;

/// The context provider for internationalization.
pub struct I18nProvider {
    pub current_locale: Signal<Locale>,
}

impl I18nProvider {
    pub fn new(initial: Locale) -> Self {
        Self {
            current_locale: Signal::new(initial),
        }
    }
}
