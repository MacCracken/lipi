//! Integration tests for lipi — cross-module behavior.

use lipi::phoneme::{self, PhonemeKind, StressPattern};
use lipi::script::{Direction, Script, ScriptType};
use lipi::grammar::{GrammarProfile, Morphology, WordOrder};
use lipi::lexicon::{LexEntry, Lexicon, PartOfSpeech};

#[test]
fn test_english_phoneme_serde_roundtrip() {
    let en = phoneme::english();
    let json = serde_json::to_string(&en).unwrap();
    let deserialized: phoneme::PhonemeInventory = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.language_code, "en");
    assert_eq!(deserialized.phonemes.len(), en.phonemes.len());
    assert_eq!(deserialized.stress, StressPattern::Free);
}

#[test]
fn test_script_serde_roundtrip() {
    let latin = Script {
        code: "Latn".into(),
        name: "Latin".into(),
        script_type: ScriptType::Alphabet,
        direction: Direction::LeftToRight,
        unicode_ranges: vec![(0x0041, 0x005A), (0x0061, 0x007A)],
        languages: vec!["en".into(), "fr".into()],
    };
    let json = serde_json::to_string(&latin).unwrap();
    let deserialized: Script = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.code, "Latn");
    assert_eq!(deserialized.script_type, ScriptType::Alphabet);
}

#[test]
fn test_grammar_serde_roundtrip() {
    let en = GrammarProfile {
        language_code: "en".into(),
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
}

#[test]
fn test_lexicon_serde_roundtrip() {
    let lex = Lexicon {
        language_code: "en".into(),
        entries: vec![
            LexEntry {
                word: "water".into(),
                ipa: "ˈwɔːtər".into(),
                gloss: "water".into(),
                pos: PartOfSpeech::Noun,
                frequency_rank: Some(250),
                swadesh_index: Some(1),
            },
        ],
    };
    let json = serde_json::to_string(&lex).unwrap();
    let deserialized: Lexicon = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.entries.len(), 1);
    assert_eq!(deserialized.find("water").unwrap().ipa, "ˈwɔːtər");
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
    // All phonemes must be either consonant or vowel
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
