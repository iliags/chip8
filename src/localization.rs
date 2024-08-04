#![allow(missing_docs)]
use fluent_templates::static_loader;
use unic_langid::{langid, LanguageIdentifier};

// TODO: Localization
const US_ENGLISH: LanguageIdentifier = langid!("en-US");
/// The language to use for localization
pub const LANG: LanguageIdentifier = US_ENGLISH;

#[cfg(debug_assertions)]
static_loader! {
    pub static LOCALES = {
        locales: "./locales",
        fallback_language: "en-US",
        // Should only set to false when testing.
        customise: |bundle| bundle.set_use_isolating(false),
    };
}

/// The static loader for localization
#[cfg(not(debug_assertions))]
static_loader! {
    pub static LOCALES = {
        locales: "./locales",
        fallback_language: "en-US",
    };
}
