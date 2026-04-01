//! Grammar — morphological typology, word order, case systems.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// Morphological typology.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

/// Look up a grammar profile by ISO 639 code.
#[must_use]
pub fn by_code(code: &str) -> Option<GrammarProfile> {
    tracing::trace!(code, "grammar profile lookup");
    match code {
        "ar" => Some(arabic()),
        "zh" => Some(mandarin()),
        "hi" => Some(hindi()),
        "ja" => Some(japanese()),
        "es" => Some(spanish()),
        "fr" => Some(french()),
        "de" => Some(german()),
        "ru" => Some(russian()),
        "ko" => Some(korean()),
        "pt" => Some(portuguese()),
        _ => None,
    }
}

/// All language codes with grammar profiles.
#[must_use]
pub fn all_codes() -> &'static [&'static str] {
    &["ar", "zh", "hi", "ja", "es", "fr", "de", "ru", "ko", "pt"]
}

// ---------------------------------------------------------------------------
// Pre-built grammar profiles
// ---------------------------------------------------------------------------

/// Arabic grammar profile.
#[must_use]
pub fn arabic() -> GrammarProfile {
    GrammarProfile {
        language_code: Cow::Borrowed("ar"),
        morphology: Morphology::Fusional,
        word_order: WordOrder::VSO,
        case_count: 3, // nominative, accusative, genitive
        has_gender: true,
        gender_count: 2,
        has_dual: true,
        has_classifiers: false,
    }
}

/// Mandarin Chinese grammar profile.
#[must_use]
pub fn mandarin() -> GrammarProfile {
    GrammarProfile {
        language_code: Cow::Borrowed("zh"),
        morphology: Morphology::Isolating,
        word_order: WordOrder::SVO,
        case_count: 0,
        has_gender: false,
        gender_count: 0,
        has_dual: false,
        has_classifiers: true,
    }
}

/// Hindi grammar profile.
#[must_use]
pub fn hindi() -> GrammarProfile {
    GrammarProfile {
        language_code: Cow::Borrowed("hi"),
        morphology: Morphology::Fusional,
        word_order: WordOrder::SOV,
        case_count: 3, // direct, oblique, vocative
        has_gender: true,
        gender_count: 2,
        has_dual: false,
        has_classifiers: false,
    }
}

/// Japanese grammar profile.
#[must_use]
pub fn japanese() -> GrammarProfile {
    GrammarProfile {
        language_code: Cow::Borrowed("ja"),
        morphology: Morphology::Agglutinative,
        word_order: WordOrder::SOV,
        case_count: 0, // particles, not inflectional case
        has_gender: false,
        gender_count: 0,
        has_dual: false,
        has_classifiers: true,
    }
}

/// Spanish grammar profile.
#[must_use]
pub fn spanish() -> GrammarProfile {
    GrammarProfile {
        language_code: Cow::Borrowed("es"),
        morphology: Morphology::Fusional,
        word_order: WordOrder::SVO,
        case_count: 0, // no case on nouns (pronoun case only)
        has_gender: true,
        gender_count: 2,
        has_dual: false,
        has_classifiers: false,
    }
}

/// French grammar profile.
#[must_use]
pub fn french() -> GrammarProfile {
    GrammarProfile {
        language_code: Cow::Borrowed("fr"),
        morphology: Morphology::Fusional,
        word_order: WordOrder::SVO,
        case_count: 0,
        has_gender: true,
        gender_count: 2,
        has_dual: false,
        has_classifiers: false,
    }
}

/// German grammar profile.
#[must_use]
pub fn german() -> GrammarProfile {
    GrammarProfile {
        language_code: Cow::Borrowed("de"),
        morphology: Morphology::Fusional,
        word_order: WordOrder::SVO, // V2 in main clauses, SOV in subordinates
        case_count: 4,              // nominative, accusative, dative, genitive
        has_gender: true,
        gender_count: 3, // masculine, feminine, neuter
        has_dual: false,
        has_classifiers: false,
    }
}

/// Russian grammar profile.
#[must_use]
pub fn russian() -> GrammarProfile {
    GrammarProfile {
        language_code: Cow::Borrowed("ru"),
        morphology: Morphology::Fusional,
        word_order: WordOrder::SVO, // flexible, but SVO dominant
        case_count: 6, // nominative, accusative, genitive, dative, instrumental, prepositional
        has_gender: true,
        gender_count: 3,
        has_dual: false,
        has_classifiers: false,
    }
}

/// Korean grammar profile.
#[must_use]
pub fn korean() -> GrammarProfile {
    GrammarProfile {
        language_code: Cow::Borrowed("ko"),
        morphology: Morphology::Agglutinative,
        word_order: WordOrder::SOV,
        case_count: 0, // particles, not inflectional case
        has_gender: false,
        gender_count: 0,
        has_dual: false,
        has_classifiers: true,
    }
}

/// Portuguese grammar profile.
#[must_use]
pub fn portuguese() -> GrammarProfile {
    GrammarProfile {
        language_code: Cow::Borrowed("pt"),
        morphology: Morphology::Fusional,
        word_order: WordOrder::SVO,
        case_count: 0,
        has_gender: true,
        gender_count: 2,
        has_dual: false,
        has_classifiers: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! grammar_test {
        ($name:ident, $fn:ident, $code:expr, $morph:expr, $order:expr) => {
            #[test]
            fn $name() {
                let g = $fn();
                assert_eq!(g.language_code, $code);
                assert_eq!(g.morphology, $morph);
                assert_eq!(g.word_order, $order);
            }
        };
    }

    grammar_test!(
        test_arabic,
        arabic,
        "ar",
        Morphology::Fusional,
        WordOrder::VSO
    );
    grammar_test!(
        test_mandarin,
        mandarin,
        "zh",
        Morphology::Isolating,
        WordOrder::SVO
    );
    grammar_test!(
        test_hindi,
        hindi,
        "hi",
        Morphology::Fusional,
        WordOrder::SOV
    );
    grammar_test!(
        test_japanese,
        japanese,
        "ja",
        Morphology::Agglutinative,
        WordOrder::SOV
    );
    grammar_test!(
        test_spanish,
        spanish,
        "es",
        Morphology::Fusional,
        WordOrder::SVO
    );
    grammar_test!(
        test_french,
        french,
        "fr",
        Morphology::Fusional,
        WordOrder::SVO
    );
    grammar_test!(
        test_german,
        german,
        "de",
        Morphology::Fusional,
        WordOrder::SVO
    );
    grammar_test!(
        test_russian,
        russian,
        "ru",
        Morphology::Fusional,
        WordOrder::SVO
    );
    grammar_test!(
        test_korean,
        korean,
        "ko",
        Morphology::Agglutinative,
        WordOrder::SOV
    );
    grammar_test!(
        test_portuguese,
        portuguese,
        "pt",
        Morphology::Fusional,
        WordOrder::SVO
    );

    #[test]
    fn test_german_cases() {
        let de = german();
        assert_eq!(de.case_count, 4);
        assert_eq!(de.gender_count, 3);
    }

    #[test]
    fn test_russian_cases() {
        let ru = russian();
        assert_eq!(ru.case_count, 6);
        assert_eq!(ru.gender_count, 3);
    }

    #[test]
    fn test_arabic_dual() {
        let ar = arabic();
        assert!(ar.has_dual);
    }

    #[test]
    fn test_classifiers() {
        assert!(mandarin().has_classifiers);
        assert!(japanese().has_classifiers);
        assert!(korean().has_classifiers);
        assert!(!spanish().has_classifiers);
    }

    #[test]
    fn test_by_code_lookup() {
        assert!(by_code("ar").is_some());
        assert!(by_code("zh").is_some());
        assert!(by_code("xx").is_none());
    }

    #[test]
    fn test_all_codes_have_profiles() {
        for code in all_codes() {
            assert!(by_code(code).is_some(), "missing grammar for {code}");
        }
    }

    #[test]
    fn test_grammar_serde_roundtrip() {
        for code in all_codes() {
            let g = by_code(code).unwrap();
            let json = serde_json::to_string(&g).unwrap();
            let back: GrammarProfile = serde_json::from_str(&json).unwrap();
            assert_eq!(g, back, "serde roundtrip failed for {code}");
        }
    }
}
