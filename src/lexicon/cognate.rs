//! Cognate detection and etymology tracking.
//!
//! Cognates are words in different languages that share a common ancestor.
//! This module models cognate sets and etymological information for
//! tracking loanwords and inherited vocabulary.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// A set of cognate words across languages.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CognateSet {
    /// Reconstructed proto-form (e.g., PIE *wódr̥ for "water").
    pub proto_form: Option<Cow<'static, str>>,
    /// Proto-language (e.g., "Proto-Indo-European").
    pub proto_language: Option<Cow<'static, str>>,
    /// Semantic field (e.g., "water", "fire", "kinship").
    pub gloss: Cow<'static, str>,
    /// Cognate entries across languages.
    pub entries: Vec<CognateEntry>,
}

/// A single word in a cognate set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CognateEntry {
    /// ISO 639 language code.
    pub language: Cow<'static, str>,
    /// The word in its native form.
    pub word: Cow<'static, str>,
    /// IPA transcription.
    pub ipa: Cow<'static, str>,
}

impl CognateSet {
    /// Get the cognate entry for a specific language.
    #[must_use]
    pub fn for_language(&self, code: &str) -> Option<&CognateEntry> {
        self.entries.iter().find(|e| e.language == code)
    }

    /// Number of languages represented in this cognate set.
    #[must_use]
    #[inline]
    pub fn language_count(&self) -> usize {
        self.entries.len()
    }
}

/// Etymology information for a lexical entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Etymology {
    /// Source language (e.g., "fr" for a French loanword in English).
    pub source_language: Cow<'static, str>,
    /// Original form in the source language.
    pub source_form: Cow<'static, str>,
    /// How the word was borrowed.
    pub borrowing_type: BorrowingType,
    /// Approximate period (e.g., "Middle English", "20th century").
    pub period: Option<Cow<'static, str>>,
}

/// How a word was borrowed from another language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum BorrowingType {
    /// Direct phonological borrowing (e.g., "café" from French).
    Loanword,
    /// Morpheme-by-morpheme translation (e.g., "skyscraper" → German "Wolkenkratzer").
    Calque,
    /// Meaning shift influenced by another language.
    SemanticLoan,
    /// Inherited from a common ancestor (not borrowed).
    Inherited,
}

// ---------------------------------------------------------------------------
// Pre-built cognate sets
// ---------------------------------------------------------------------------

/// Water cognates across Indo-European languages.
#[must_use]
pub fn water_cognates() -> CognateSet {
    CognateSet {
        proto_form: Some(Cow::Borrowed("*wódr̥")),
        proto_language: Some(Cow::Borrowed("Proto-Indo-European")),
        gloss: Cow::Borrowed("water"),
        entries: vec![
            CognateEntry {
                language: Cow::Borrowed("en"),
                word: Cow::Borrowed("water"),
                ipa: Cow::Borrowed("ˈwɔːtər"),
            },
            CognateEntry {
                language: Cow::Borrowed("de"),
                word: Cow::Borrowed("Wasser"),
                ipa: Cow::Borrowed("ˈvasɐ"),
            },
            CognateEntry {
                language: Cow::Borrowed("ru"),
                word: Cow::Borrowed("вода"),
                ipa: Cow::Borrowed("vɐˈda"),
            },
            CognateEntry {
                language: Cow::Borrowed("sa"),
                word: Cow::Borrowed("उदन्"),
                ipa: Cow::Borrowed("udán"),
            },
            CognateEntry {
                language: Cow::Borrowed("el"),
                word: Cow::Borrowed("ύδωρ"),
                ipa: Cow::Borrowed("ˈiðor"),
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_water_cognates() {
        let cog = water_cognates();
        assert_eq!(cog.language_count(), 5);
        assert_eq!(cog.proto_form.as_deref(), Some("*wódr̥"));
        let en = cog.for_language("en").unwrap();
        assert_eq!(en.word, "water");
    }

    #[test]
    fn test_cognate_unknown_language() {
        let cog = water_cognates();
        assert!(cog.for_language("xx").is_none());
    }

    #[test]
    fn test_cognate_serde_roundtrip() {
        let cog = water_cognates();
        let json = serde_json::to_string(&cog).unwrap();
        let back: CognateSet = serde_json::from_str(&json).unwrap();
        assert_eq!(cog, back);
    }

    #[test]
    fn test_etymology_serde_roundtrip() {
        let etym = Etymology {
            source_language: Cow::Borrowed("fr"),
            source_form: Cow::Borrowed("café"),
            borrowing_type: BorrowingType::Loanword,
            period: Some(Cow::Borrowed("18th century")),
        };
        let json = serde_json::to_string(&etym).unwrap();
        let back: Etymology = serde_json::from_str(&json).unwrap();
        assert_eq!(etym, back);
    }

    #[test]
    fn test_borrowing_types() {
        assert_ne!(BorrowingType::Loanword, BorrowingType::Calque);
        assert_ne!(BorrowingType::SemanticLoan, BorrowingType::Inherited);
    }
}
