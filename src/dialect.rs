//! Dialect and language variety support.
//!
//! Languages have regional and social varieties that differ in phonology,
//! vocabulary, and grammar. This module models those differences as
//! overlays on a base language inventory.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::phoneme::allophone::AllophoneRule;

/// A language variety (dialect, accent, register).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanguageVariety {
    /// Variety identifier (e.g., "en-GB", "ar-EG", "zh-yue").
    pub code: Cow<'static, str>,
    /// Human-readable name (e.g., "British English", "Egyptian Arabic").
    pub name: Cow<'static, str>,
    /// Parent language code (e.g., "en" for "en-GB").
    pub parent: Cow<'static, str>,
    /// What kind of variety this is.
    pub kind: VarietyKind,
    /// Phonemes added relative to the parent inventory.
    pub added_phonemes: Vec<Cow<'static, str>>,
    /// Phonemes removed relative to the parent inventory.
    pub removed_phonemes: Vec<Cow<'static, str>>,
    /// Additional allophone rules specific to this variety.
    pub allophone_overrides: Vec<AllophoneRule>,
}

/// Classification of language varieties.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum VarietyKind {
    /// Regional dialect (e.g., Cockney, Bavarian).
    Regional,
    /// National standard (e.g., British English, Brazilian Portuguese).
    NationalStandard,
    /// Historical stage (e.g., Middle English, Classical Arabic).
    Historical,
    /// Sociolect or register (e.g., African American Vernacular English).
    Sociolect,
    /// Creole or pidgin derived from this language.
    Creole,
}

impl LanguageVariety {
    /// Check if this variety adds a phoneme not in the parent.
    #[must_use]
    pub fn adds(&self, ipa: &str) -> bool {
        self.added_phonemes.iter().any(|p| p.as_ref() == ipa)
    }

    /// Check if this variety removes a phoneme from the parent.
    #[must_use]
    pub fn removes(&self, ipa: &str) -> bool {
        self.removed_phonemes.iter().any(|p| p.as_ref() == ipa)
    }
}

// ---------------------------------------------------------------------------
// Pre-built varieties
// ---------------------------------------------------------------------------

/// British English (Received Pronunciation).
#[must_use]
pub fn british_english() -> LanguageVariety {
    use crate::phoneme::allophone::{AllophoneRule, Environment};

    LanguageVariety {
        code: Cow::Borrowed("en-GB"),
        name: Cow::Borrowed("British English (RP)"),
        parent: Cow::Borrowed("en"),
        kind: VarietyKind::NationalStandard,
        added_phonemes: vec![
            Cow::Borrowed("ɒ"), // LOT vowel (not in GA)
        ],
        removed_phonemes: vec![
            Cow::Borrowed("ɹ"), // non-rhotic: no post-vocalic /ɹ/
        ],
        allophone_overrides: vec![
            // No flapping — /t/ stays [t] intervocalically in RP
            AllophoneRule {
                phoneme: Cow::Borrowed("t"),
                allophone: Cow::Borrowed("t"),
                environment: Environment::Intervocalic,
                obligatory: true,
            },
        ],
    }
}

/// Egyptian Arabic (Cairene).
#[must_use]
pub fn egyptian_arabic() -> LanguageVariety {
    LanguageVariety {
        code: Cow::Borrowed("ar-EG"),
        name: Cow::Borrowed("Egyptian Arabic"),
        parent: Cow::Borrowed("ar"),
        kind: VarietyKind::Regional,
        added_phonemes: vec![],
        removed_phonemes: vec![],
        allophone_overrides: vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_british_english() {
        let rp = british_english();
        assert_eq!(rp.parent, "en");
        assert_eq!(rp.kind, VarietyKind::NationalStandard);
        assert!(rp.adds("ɒ"));
        assert!(rp.removes("ɹ"));
        assert!(!rp.adds("xyz"));
    }

    #[test]
    fn test_variety_serde_roundtrip() {
        let rp = british_english();
        let json = serde_json::to_string(&rp).unwrap();
        let back: LanguageVariety = serde_json::from_str(&json).unwrap();
        assert_eq!(rp, back);
    }

    #[test]
    fn test_egyptian_arabic() {
        let eg = egyptian_arabic();
        assert_eq!(eg.parent, "ar");
        assert_eq!(eg.kind, VarietyKind::Regional);
    }
}
