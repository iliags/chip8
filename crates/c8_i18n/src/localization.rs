// The static loader macro doesn't work with documentation comments, so we have to disable the warning.
#![allow(missing_docs)]
use fluent_templates::static_loader;
use unic_langid::{langid, LanguageIdentifier};

/// The languages available for localization
#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub enum Languages {
    English,
}

/// Array of available languages for iteration
pub const LANGUAGE_LIST: [Languages; 1] = [Languages::English];

impl Default for Languages {
    fn default() -> Self {
        Self::English
    }
}

impl Languages {
    pub fn value(&self) -> LanguageIdentifier {
        match self {
            Self::English => langid!("en-US"),
        }
    }

    // Note: Make sure the text matches the native form (i.e. "Français" for French)
    pub fn as_str(&self) -> &str {
        match self {
            Self::English => "English",
        }
    }
}

#[cfg(debug_assertions)]
static_loader! {
    pub static LOCALES = {
        locales: "locales",
        fallback_language: "en-US",
        // Should only set to false when testing.
        customise: |bundle| bundle.set_use_isolating(false),
    };
}

#[cfg(not(debug_assertions))]
static_loader! {
    pub static LOCALES = {
        locales: "locales",
        fallback_language: "en-US",
    };
}
