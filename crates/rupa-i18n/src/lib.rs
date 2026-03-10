pub mod provider;
pub mod dictionary;
pub mod locale;

pub use provider::I18nProvider;
pub use locale::Locale;

/// Translates a key into the current language reactively.
pub fn translate(_key: &str) -> String {
    todo!("i18n translation not yet implemented")
}
