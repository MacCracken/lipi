//! Grammar — morphological typology, word order, case systems.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// Morphological typology.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Morphology {
    /// Words are mostly invariant, grammar via word order (e.g., Mandarin, Vietnamese).
    Isolating,
    /// Words built by chaining morphemes (e.g., Turkish, Finnish, Japanese).
    Agglutinative,
    /// Morphemes fuse together, one affix = multiple meanings (e.g., Latin, Russian).
    Fusional,
    /// Entire sentences in one word (e.g., Mohawk, Yupik).
    Polysynthetic,
}

/// Dominant word order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum WordOrder {
    SVO,  // English, French, Mandarin
    SOV,  // Japanese, Korean, Hindi, Turkish
    VSO,  // Arabic, Irish, Welsh
    VOS,  // Malagasy, Fijian
    OVS,  // Hixkaryana
    OSV,  // Rare
    Free, // Latin, Russian (case-marked)
}

/// A language's grammatical profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GrammarProfile {
    /// ISO 639 language code.
    pub language_code: Cow<'static, str>,
    /// Morphological typology.
    pub morphology: Morphology,
    /// Dominant word order.
    pub word_order: WordOrder,
    /// Number of grammatical cases (0 = no case system).
    pub case_count: u8,
    /// Has grammatical gender.
    pub has_gender: bool,
    /// Number of genders (0 if none).
    pub gender_count: u8,
    /// Has grammatical number beyond singular/plural.
    pub has_dual: bool,
    /// Uses classifier/measure words (e.g., Mandarin, Japanese).
    pub has_classifiers: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_english_grammar() {
        let en = GrammarProfile {
            language_code: Cow::Borrowed("en"),
            morphology: Morphology::Fusional,
            word_order: WordOrder::SVO,
            case_count: 2,     // subjective/objective (I/me, he/him)
            has_gender: false, // no grammatical gender on nouns
            gender_count: 0,
            has_dual: false,
            has_classifiers: false,
        };
        assert_eq!(en.word_order, WordOrder::SVO);
    }

    #[test]
    fn test_japanese_grammar() {
        let ja = GrammarProfile {
            language_code: Cow::Borrowed("ja"),
            morphology: Morphology::Agglutinative,
            word_order: WordOrder::SOV,
            case_count: 0, // particles, not case inflection
            has_gender: false,
            gender_count: 0,
            has_dual: false,
            has_classifiers: true,
        };
        assert_eq!(ja.morphology, Morphology::Agglutinative);
        assert!(ja.has_classifiers);
    }

    #[test]
    fn test_arabic_grammar() {
        let ar = GrammarProfile {
            language_code: Cow::Borrowed("ar"),
            morphology: Morphology::Fusional,
            word_order: WordOrder::VSO,
            case_count: 3, // nominative, accusative, genitive
            has_gender: true,
            gender_count: 2,
            has_dual: true, // Arabic has dual number
            has_classifiers: false,
        };
        assert!(ar.has_dual);
        assert_eq!(ar.case_count, 3);
    }
}
