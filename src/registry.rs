//! Language registry — look up phoneme inventories and scripts by ISO 639 code.
//!
//! Central entry point for querying language data. Returns pre-built inventories
//! and script metadata for registered languages.

use crate::phoneme::PhonemeInventory;
use crate::script::Script;

/// A registered language's available data.
#[derive(Debug, Clone)]
pub struct LanguageInfo {
    /// ISO 639-1 or 639-3 language code.
    pub code: &'static str,
    /// Language name in English.
    pub name: &'static str,
    /// ISO 15924 script codes used by this language.
    pub script_codes: &'static [&'static str],
}

/// All registered language codes.
pub const REGISTERED: &[LanguageInfo] = &[
    LanguageInfo {
        code: "en",
        name: "English",
        script_codes: &["Latn"],
    },
    LanguageInfo {
        code: "sa",
        name: "Sanskrit",
        script_codes: &["Deva"],
    },
    LanguageInfo {
        code: "el",
        name: "Greek",
        script_codes: &["Grek"],
    },
];

/// Look up a language by ISO 639 code.
#[must_use]
pub fn info(code: &str) -> Option<&'static LanguageInfo> {
    tracing::trace!(code, "language info lookup");
    REGISTERED.iter().find(|l| l.code == code)
}

/// Get the phoneme inventory for a language, if available.
#[must_use]
pub fn phonemes(code: &str) -> Option<PhonemeInventory> {
    tracing::trace!(code, "phoneme inventory lookup");
    match code {
        "en" => Some(crate::phoneme::english()),
        "sa" => Some(crate::phoneme::sanskrit()),
        "el" => Some(crate::phoneme::greek()),
        _ => None,
    }
}

/// Get the primary script for a language, if available.
#[must_use]
pub fn primary_script(code: &str) -> Option<Script> {
    tracing::trace!(code, "primary script lookup");
    info(code).and_then(|l| {
        l.script_codes
            .first()
            .and_then(|sc| crate::script::by_code(sc))
    })
}

/// All registered ISO 639 language codes.
#[must_use]
pub fn all_codes() -> Vec<&'static str> {
    REGISTERED.iter().map(|l| l.code).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_info_lookup() {
        let en = info("en").unwrap();
        assert_eq!(en.name, "English");
        assert_eq!(en.script_codes, &["Latn"]);
    }

    #[test]
    fn test_info_unknown() {
        assert!(info("xx").is_none());
    }

    #[test]
    fn test_phonemes_lookup() {
        let sa = phonemes("sa").unwrap();
        assert_eq!(sa.language_code, "sa");
        assert!(sa.consonant_count() > 0);
    }

    #[test]
    fn test_phonemes_unknown() {
        assert!(phonemes("xx").is_none());
    }

    #[test]
    fn test_primary_script() {
        let script = primary_script("en").unwrap();
        assert_eq!(script.code, "Latn");
    }

    #[test]
    fn test_primary_script_sanskrit() {
        let script = primary_script("sa").unwrap();
        assert_eq!(script.code, "Deva");
    }

    #[test]
    fn test_primary_script_greek() {
        let script = primary_script("el").unwrap();
        assert_eq!(script.code, "Grek");
    }

    #[test]
    fn test_primary_script_unknown() {
        assert!(primary_script("xx").is_none());
    }

    #[test]
    fn test_all_codes() {
        let codes = all_codes();
        assert!(codes.contains(&"en"));
        assert!(codes.contains(&"sa"));
        assert!(codes.contains(&"el"));
    }

    #[test]
    fn test_all_registered_have_phonemes() {
        for lang in REGISTERED {
            assert!(
                phonemes(lang.code).is_some(),
                "missing phoneme inventory for {}",
                lang.code
            );
        }
    }

    #[test]
    fn test_all_registered_have_scripts() {
        for lang in REGISTERED {
            assert!(
                primary_script(lang.code).is_some(),
                "missing script for {}",
                lang.code
            );
        }
    }
}
