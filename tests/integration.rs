//! Integration tests for lipi — cross-module behavior.

use std::borrow::Cow;

use lipi::grammar::{GrammarProfile, Morphology, WordOrder};
use lipi::lexicon::{LexEntry, Lexicon, PartOfSpeech};
use lipi::phoneme::{self, Backness, Height, Manner, Phoneme, PhonemeKind, Place, StressPattern};
use lipi::script::{Direction, Script, ScriptType};

#[test]
fn test_english_phoneme_serde_roundtrip() {
    let en = phoneme::english();
    let json = serde_json::to_string(&en).unwrap();
    let deserialized: phoneme::PhonemeInventory = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.language_code, "en");
    assert_eq!(deserialized.phonemes.len(), en.phonemes.len());
    assert_eq!(deserialized.stress, StressPattern::Free);
    // Cow roundtrip: borrowed → serialize → deserialize → owned, but still equal
    assert_eq!(deserialized, en);
}

#[test]
fn test_script_serde_roundtrip() {
    let latin = Script {
        code: Cow::Borrowed("Latn"),
        name: Cow::Borrowed("Latin"),
        script_type: ScriptType::Alphabet,
        direction: Direction::LeftToRight,
        unicode_ranges: vec![(0x0041, 0x005A), (0x0061, 0x007A)],
        languages: vec![Cow::Borrowed("en"), Cow::Borrowed("fr")],
    };
    let json = serde_json::to_string(&latin).unwrap();
    let deserialized: Script = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.code, "Latn");
    assert_eq!(deserialized.script_type, ScriptType::Alphabet);
    assert_eq!(deserialized, latin);
}

#[test]
fn test_grammar_serde_roundtrip() {
    let en = GrammarProfile {
        language_code: Cow::Borrowed("en"),
        morphology: Morphology::Fusional,
        word_order: WordOrder::SVO,
        case_count: 2,
        has_gender: false,
        gender_count: 0,
        has_dual: false,
        has_classifiers: false,
    };
    let json = serde_json::to_string(&en).unwrap();
    let deserialized: GrammarProfile = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.word_order, WordOrder::SVO);
    assert_eq!(deserialized.morphology, Morphology::Fusional);
    assert_eq!(deserialized, en);
}

#[test]
fn test_lexicon_serde_roundtrip() {
    let lex = Lexicon {
        language_code: Cow::Borrowed("en"),
        entries: vec![LexEntry {
            word: Cow::Borrowed("water"),
            ipa: Cow::Borrowed("ˈwɔːtər"),
            gloss: Cow::Borrowed("water"),
            pos: PartOfSpeech::Noun,
            frequency_rank: Some(250),
            swadesh_index: Some(1),
        }],
    };
    let json = serde_json::to_string(&lex).unwrap();
    let deserialized: Lexicon = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.entries.len(), 1);
    assert_eq!(deserialized.find("water").unwrap().ipa, "ˈwɔːtər");
    assert_eq!(deserialized, lex);
}

#[test]
fn test_english_consonant_vowel_split() {
    let en = phoneme::english();
    let total = en.phonemes.len();
    let consonants = en.consonant_count();
    let vowels = en.vowel_count();
    assert_eq!(consonants + vowels, total);
}

#[test]
fn test_phoneme_kind_classification() {
    let en = phoneme::english();
    for p in &en.phonemes {
        match &p.kind {
            PhonemeKind::Consonant { .. } => {}
            PhonemeKind::Vowel { .. } => {}
            _ => panic!("unexpected phoneme kind for /{}/", p.ipa),
        }
    }
}

#[test]
fn test_error_display() {
    let err = lipi::LipiError::UnknownLanguage("xx".into());
    assert_eq!(err.to_string(), "unknown language: xx");

    let err = lipi::LipiError::PhonemeNotInInventory {
        phoneme: "ʀ".into(),
        language: "en".into(),
    };
    assert!(err.to_string().contains("ʀ"));
    assert!(err.to_string().contains("en"));
}

#[test]
fn test_all_error_variants_display() {
    let cases: Vec<(lipi::LipiError, &str)> = vec![
        (
            lipi::LipiError::UnknownLanguage("zz".into()),
            "unknown language: zz",
        ),
        (
            lipi::LipiError::UnknownScript("Xxxx".into()),
            "unknown script: Xxxx",
        ),
        (
            lipi::LipiError::InvalidIpa("???".into()),
            "invalid IPA symbol: ???",
        ),
        (
            lipi::LipiError::WordNotFound {
                word: "foo".into(),
                language: "en".into(),
            },
            "word not found: foo in en",
        ),
    ];
    for (err, expected) in cases {
        assert_eq!(err.to_string(), expected);
    }
}

#[test]
fn test_phoneme_kind_serde_consonant() {
    let p = Phoneme::consonant("p", Manner::Plosive, Place::Bilabial, false);
    let json = serde_json::to_string(&p).unwrap();
    let back: Phoneme = serde_json::from_str(&json).unwrap();
    assert_eq!(p, back);
}

#[test]
fn test_phoneme_kind_serde_vowel() {
    let p = Phoneme::vowel("æ", Height::NearOpen, Backness::Front, false);
    let json = serde_json::to_string(&p).unwrap();
    let back: Phoneme = serde_json::from_str(&json).unwrap();
    assert_eq!(p, back);
}

#[test]
fn test_phoneme_constructors() {
    let c = Phoneme::consonant("t", Manner::Plosive, Place::Alveolar, false);
    assert_eq!(c.ipa, "t");
    assert!(matches!(
        c.kind,
        PhonemeKind::Consonant { voiced: false, .. }
    ));

    let v = Phoneme::vowel("iː", Height::Close, Backness::Front, false);
    assert_eq!(v.ipa, "iː");
    assert!(matches!(v.kind, PhonemeKind::Vowel { rounded: false, .. }));
}

#[test]
fn test_phoneme_constructor_with_owned_string() {
    let ipa = String::from("custom");
    let p = Phoneme::consonant(ipa, Manner::Fricative, Place::Glottal, false);
    assert_eq!(p.ipa, "custom");
}
