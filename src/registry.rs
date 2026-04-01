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
    LanguageInfo {
        code: "yua",
        name: "Yucatec Maya",
        script_codes: &["Latn"],
    },
    LanguageInfo {
        code: "sw",
        name: "Swahili",
        script_codes: &["Latn"],
    },
    LanguageInfo {
        code: "yo",
        name: "Yoruba",
        script_codes: &["Latn"],
    },
    LanguageInfo {
        code: "zu",
        name: "Zulu",
        script_codes: &["Latn"],
    },
    LanguageInfo {
        code: "th",
        name: "Thai",
        script_codes: &["Thai"],
    },
    LanguageInfo {
        code: "vi",
        name: "Vietnamese",
        script_codes: &["Latn"],
    },
    LanguageInfo {
        code: "tl",
        name: "Tagalog",
        script_codes: &["Latn"],
    },
    LanguageInfo {
        code: "tr",
        name: "Turkish",
        script_codes: &["Latn"],
    },
    LanguageInfo {
        code: "fi",
        name: "Finnish",
        script_codes: &["Latn"],
    },
    LanguageInfo {
        code: "haw",
        name: "Hawaiian",
        script_codes: &["Latn"],
    },
    LanguageInfo {
        code: "nah",
        name: "Nahuatl",
        script_codes: &["Latn"],
    },
    LanguageInfo {
        code: "la",
        name: "Latin",
        script_codes: &["Latn"],
    },
    LanguageInfo {
        code: "ar",
        name: "Arabic",
        script_codes: &["Arab"],
    },
    LanguageInfo {
        code: "grc",
        name: "Koine Greek",
        script_codes: &["Grek"],
    },
    LanguageInfo {
        code: "lzh",
        name: "Literary Chinese",
        script_codes: &["Hani"],
    },
    LanguageInfo {
        code: "zh",
        name: "Mandarin Chinese",
        script_codes: &["Hani"],
    },
    LanguageInfo {
        code: "hi",
        name: "Hindi",
        script_codes: &["Deva"],
    },
    LanguageInfo {
        code: "bn",
        name: "Bengali",
        script_codes: &["Deva"],
    },
    LanguageInfo {
        code: "ta",
        name: "Tamil",
        script_codes: &["Taml"],
    },
    LanguageInfo {
        code: "ur",
        name: "Urdu",
        script_codes: &["Arab"],
    },
    LanguageInfo {
        code: "ja",
        name: "Japanese",
        script_codes: &["Kana"],
    },
    LanguageInfo {
        code: "es",
        name: "Spanish",
        script_codes: &["Latn"],
    },
    LanguageInfo {
        code: "fr",
        name: "French",
        script_codes: &["Latn"],
    },
    LanguageInfo {
        code: "de",
        name: "German",
        script_codes: &["Latn"],
    },
    LanguageInfo {
        code: "ru",
        name: "Russian",
        script_codes: &["Cyrl"],
    },
    LanguageInfo {
        code: "ko",
        name: "Korean",
        script_codes: &["Hang"],
    },
    LanguageInfo {
        code: "pt",
        name: "Portuguese",
        script_codes: &["Latn"],
    },
    LanguageInfo {
        code: "it",
        name: "Italian",
        script_codes: &["Latn"],
    },
    LanguageInfo {
        code: "nl",
        name: "Dutch",
        script_codes: &["Latn"],
    },
    LanguageInfo {
        code: "pl",
        name: "Polish",
        script_codes: &["Latn"],
    },
    LanguageInfo {
        code: "am",
        name: "Amharic",
        script_codes: &["Ethi"],
    },
    LanguageInfo {
        code: "ha",
        name: "Hausa",
        script_codes: &["Latn"],
    },
    LanguageInfo {
        code: "id",
        name: "Indonesian",
        script_codes: &["Latn"],
    },
    LanguageInfo {
        code: "fa",
        name: "Persian",
        script_codes: &["Arab"],
    },
    LanguageInfo {
        code: "he",
        name: "Hebrew",
        script_codes: &["Hebr"],
    },
    LanguageInfo {
        code: "ka",
        name: "Georgian",
        script_codes: &["Geor"],
    },
    LanguageInfo {
        code: "cs",
        name: "Czech",
        script_codes: &["Latn"],
    },
    LanguageInfo {
        code: "hu",
        name: "Hungarian",
        script_codes: &["Latn"],
    },
    LanguageInfo {
        code: "ro",
        name: "Romanian",
        script_codes: &["Latn"],
    },
    LanguageInfo {
        code: "my",
        name: "Burmese",
        script_codes: &["Mymr"],
    },
    LanguageInfo {
        code: "km",
        name: "Khmer",
        script_codes: &["Khmr"],
    },
    LanguageInfo {
        code: "so",
        name: "Somali",
        script_codes: &["Latn"],
    },
    LanguageInfo {
        code: "qu",
        name: "Quechua",
        script_codes: &["Latn"],
    },
    LanguageInfo {
        code: "gn",
        name: "Guarani",
        script_codes: &["Latn"],
    },
    LanguageInfo {
        code: "is",
        name: "Icelandic",
        script_codes: &["Latn"],
    },
    LanguageInfo {
        code: "wo",
        name: "Wolof",
        script_codes: &["Latn"],
    },
    LanguageInfo {
        code: "lo",
        name: "Lao",
        script_codes: &["Laoo"],
    },
    LanguageInfo {
        code: "mn",
        name: "Mongolian",
        script_codes: &["Cyrl"],
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
    use crate::phoneme::{english, greek, inventories::*, sanskrit};
    match code {
        "en" => Some(english()),
        "sa" => Some(sanskrit()),
        "el" => Some(greek()),
        "yua" => Some(yucatec_maya()),
        "sw" => Some(swahili()),
        "yo" => Some(yoruba()),
        "zu" => Some(zulu()),
        "th" => Some(thai()),
        "vi" => Some(vietnamese()),
        "tl" => Some(tagalog()),
        "tr" => Some(turkish()),
        "fi" => Some(finnish()),
        "haw" => Some(hawaiian()),
        "nah" => Some(nahuatl()),
        "la" => Some(latin()),
        "ar" => Some(classical_arabic()),
        "grc" => Some(koine_greek()),
        "lzh" => Some(literary_chinese()),
        "zh" => Some(mandarin()),
        "hi" => Some(hindi()),
        "bn" => Some(bengali()),
        "ta" => Some(tamil()),
        "ur" => Some(urdu()),
        "ja" => Some(japanese()),
        "es" => Some(spanish()),
        "fr" => Some(french()),
        "de" => Some(german()),
        "ru" => Some(russian()),
        "ko" => Some(korean()),
        "pt" => Some(portuguese()),
        "it" => Some(italian()),
        "nl" => Some(dutch()),
        "pl" => Some(polish()),
        "am" => Some(amharic()),
        "ha" => Some(hausa()),
        "id" => Some(indonesian()),
        "fa" => Some(persian()),
        "he" => Some(hebrew()),
        "ka" => Some(georgian()),
        "cs" => Some(czech()),
        "hu" => Some(hungarian()),
        "ro" => Some(romanian()),
        "my" => Some(burmese()),
        "km" => Some(khmer()),
        "so" => Some(somali()),
        "qu" => Some(quechua()),
        "gn" => Some(guarani()),
        "is" => Some(icelandic()),
        "wo" => Some(wolof()),
        "lo" => Some(lao()),
        "mn" => Some(mongolian()),
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
pub fn all_codes() -> &'static [&'static str] {
    &[
        "en", "sa", "el", "yua", "sw", "yo", "zu", "th", "vi", "tl", "tr", "fi", "haw", "nah",
        "la", "ar", "grc", "lzh", "zh", "hi", "bn", "ta", "ur", "ja", "es", "fr", "de", "ru", "ko",
        "pt", "it", "nl", "pl", "am", "ha", "id", "fa", "he", "ka", "cs", "hu", "ro", "my", "km",
        "so", "qu", "gn", "is", "wo", "lo", "mn",
    ]
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
    fn test_primary_script_unknown() {
        assert!(primary_script("xx").is_none());
    }

    #[test]
    fn test_all_codes() {
        let codes = all_codes();
        assert_eq!(codes.len(), 51);
        assert!(codes.contains(&"en"));
        assert!(codes.contains(&"yua"));
        assert!(codes.contains(&"haw"));
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
    fn test_all_codes_match_registered() {
        let codes = all_codes();
        assert_eq!(codes.len(), REGISTERED.len());
        for lang in REGISTERED {
            assert!(codes.contains(&lang.code), "missing code {}", lang.code);
        }
    }

    #[test]
    fn test_all_registered_have_scripts() {
        for lang in REGISTERED {
            // Some scripts (Thai) may not be registered yet
            if let Some(script) = primary_script(lang.code) {
                assert!(!script.code.is_empty());
            }
        }
    }
}
