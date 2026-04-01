//! Serde roundtrip tests — systematic JSON serialization/deserialization
//! for every public type in lipi.
//!
//! Each type is serialized to JSON and deserialized back, verifying equality.

use std::borrow::Cow;

// ---------------------------------------------------------------------------
// Phoneme types
// ---------------------------------------------------------------------------

#[test]
fn serde_phoneme_consonant() {
    let p = lipi::phoneme::Phoneme::consonant(
        "p",
        lipi::phoneme::Manner::Plosive,
        lipi::phoneme::Place::Bilabial,
        false,
    );
    let json = serde_json::to_string(&p).unwrap();
    let back: lipi::phoneme::Phoneme = serde_json::from_str(&json).unwrap();
    assert_eq!(p, back);
}

#[test]
fn serde_phoneme_vowel() {
    let p = lipi::phoneme::Phoneme::vowel(
        "æ",
        lipi::phoneme::Height::NearOpen,
        lipi::phoneme::Backness::Front,
        false,
    );
    let json = serde_json::to_string(&p).unwrap();
    let back: lipi::phoneme::Phoneme = serde_json::from_str(&json).unwrap();
    assert_eq!(p, back);
}

#[test]
fn serde_phoneme_inventory_all_languages() {
    for code in lipi::registry::all_codes() {
        let inv = lipi::registry::phonemes(code).unwrap();
        let json = serde_json::to_string(&inv).unwrap();
        let back: lipi::phoneme::PhonemeInventory = serde_json::from_str(&json).unwrap();
        assert_eq!(inv, back, "roundtrip failed for {code}");
    }
}

#[test]
fn serde_phoneme_inventory_with_tones() {
    // Mandarin has tones
    let inv = lipi::registry::phonemes("zh").unwrap();
    assert!(inv.tones.is_some());
    let json = serde_json::to_string(&inv).unwrap();
    let back: lipi::phoneme::PhonemeInventory = serde_json::from_str(&json).unwrap();
    assert_eq!(inv, back);
    assert_eq!(inv.tones, back.tones);
}

#[test]
fn serde_phoneme_inventory_without_tones() {
    let inv = lipi::registry::phonemes("en").unwrap();
    assert!(inv.tones.is_none());
    let json = serde_json::to_string(&inv).unwrap();
    let back: lipi::phoneme::PhonemeInventory = serde_json::from_str(&json).unwrap();
    assert_eq!(inv.tones, back.tones);
}

// ---------------------------------------------------------------------------
// Script types
// ---------------------------------------------------------------------------

#[test]
fn serde_script_all() {
    for code in lipi::script::all_codes() {
        let script = lipi::script::by_code(code).unwrap();
        let json = serde_json::to_string(&script).unwrap();
        let back: lipi::script::Script = serde_json::from_str(&json).unwrap();
        assert_eq!(script, back, "roundtrip failed for script {code}");
    }
}

#[test]
fn serde_script_with_attestation() {
    // Cuneiform and Egyptian have attestation fields
    let script = lipi::script::cuneiform();
    assert!(script.attestation.is_some());
    let json = serde_json::to_string(&script).unwrap();
    let back: lipi::script::Script = serde_json::from_str(&json).unwrap();
    assert_eq!(script.attestation, back.attestation);
}

#[test]
fn serde_script_without_attestation() {
    let script = lipi::script::latin();
    assert!(script.attestation.is_none());
    let json = serde_json::to_string(&script).unwrap();
    let back: lipi::script::Script = serde_json::from_str(&json).unwrap();
    assert_eq!(script.attestation, back.attestation);
}

// ---------------------------------------------------------------------------
// Transliteration types
// ---------------------------------------------------------------------------

#[test]
fn serde_transliteration_devanagari_iast() {
    let table = lipi::script::transliteration::devanagari_iast();
    let json = serde_json::to_string(&table).unwrap();
    let back: lipi::script::transliteration::TransliterationTable =
        serde_json::from_str(&json).unwrap();
    assert_eq!(table, back);
}

#[test]
fn serde_transliteration_greek_beta_code() {
    let table = lipi::script::transliteration::greek_beta_code();
    let json = serde_json::to_string(&table).unwrap();
    let back: lipi::script::transliteration::TransliterationTable =
        serde_json::from_str(&json).unwrap();
    assert_eq!(table, back);
}

// ---------------------------------------------------------------------------
// Numeral types
// ---------------------------------------------------------------------------

#[test]
fn serde_numeral_system_all() {
    let systems = [
        lipi::script::numerals::devanagari_digits(),
        lipi::script::numerals::greek_isopsephy(),
        lipi::script::numerals::babylonian_sexagesimal(),
        lipi::script::numerals::egyptian_hieroglyphic(),
        lipi::script::numerals::chinese_rod_numerals(),
    ];
    for sys in &systems {
        let json = serde_json::to_string(sys).unwrap();
        let back: lipi::script::numerals::NumeralSystem = serde_json::from_str(&json).unwrap();
        assert_eq!(*sys, back, "roundtrip failed for {}", sys.name);
    }
}

#[test]
fn serde_numeral_mapping() {
    let mapping = lipi::script::numerals::NumeralMapping {
        character: Cow::Borrowed("α"),
        value: 1,
    };
    let json = serde_json::to_string(&mapping).unwrap();
    let back: lipi::script::numerals::NumeralMapping = serde_json::from_str(&json).unwrap();
    assert_eq!(mapping, back);
}

// ---------------------------------------------------------------------------
// Grammar types
// ---------------------------------------------------------------------------

#[test]
fn serde_grammar_profile_all() {
    for code in lipi::grammar::all_codes() {
        let g = lipi::grammar::by_code(code).unwrap();
        let json = serde_json::to_string(&g).unwrap();
        let back: lipi::grammar::GrammarProfile = serde_json::from_str(&json).unwrap();
        assert_eq!(g, back, "roundtrip failed for grammar {code}");
    }
}

// ---------------------------------------------------------------------------
// Lexicon types
// ---------------------------------------------------------------------------

#[test]
fn serde_lex_entry() {
    let entry = lipi::lexicon::LexEntry {
        word: Cow::Borrowed("water"),
        ipa: Cow::Borrowed("ˈwɔːtər"),
        gloss: Cow::Borrowed("water"),
        pos: lipi::lexicon::PartOfSpeech::Noun,
        frequency_rank: Some(250),
        swadesh_index: Some(23),
    };
    let json = serde_json::to_string(&entry).unwrap();
    let back: lipi::lexicon::LexEntry = serde_json::from_str(&json).unwrap();
    assert_eq!(entry, back);
}

#[test]
fn serde_lex_entry_no_optionals() {
    let entry = lipi::lexicon::LexEntry {
        word: Cow::Borrowed("xyz"),
        ipa: Cow::Borrowed("xyz"),
        gloss: Cow::Borrowed("test"),
        pos: lipi::lexicon::PartOfSpeech::Verb,
        frequency_rank: None,
        swadesh_index: None,
    };
    let json = serde_json::to_string(&entry).unwrap();
    let back: lipi::lexicon::LexEntry = serde_json::from_str(&json).unwrap();
    assert_eq!(entry, back);
}

#[test]
fn serde_lexicon() {
    let lex = lipi::lexicon::Lexicon {
        language_code: Cow::Borrowed("en"),
        entries: vec![lipi::lexicon::LexEntry {
            word: Cow::Borrowed("fire"),
            ipa: Cow::Borrowed("ˈfaɪər"),
            gloss: Cow::Borrowed("fire"),
            pos: lipi::lexicon::PartOfSpeech::Noun,
            frequency_rank: Some(800),
            swadesh_index: Some(24),
        }],
    };
    let json = serde_json::to_string(&lex).unwrap();
    let back: lipi::lexicon::Lexicon = serde_json::from_str(&json).unwrap();
    assert_eq!(lex, back);
}

#[test]
fn serde_swadesh_all() {
    for code in lipi::lexicon::swadesh::all_codes() {
        let lex = lipi::lexicon::swadesh::by_code(code).unwrap();
        let json = serde_json::to_string(&lex).unwrap();
        let back: lipi::lexicon::Lexicon = serde_json::from_str(&json).unwrap();
        assert_eq!(lex, back, "roundtrip failed for swadesh {code}");
    }
}

// ---------------------------------------------------------------------------
// Cognate & etymology types
// ---------------------------------------------------------------------------

#[test]
fn serde_cognate_set() {
    let cog = lipi::lexicon::cognate::water_cognates();
    let json = serde_json::to_string(&cog).unwrap();
    let back: lipi::lexicon::cognate::CognateSet = serde_json::from_str(&json).unwrap();
    assert_eq!(cog, back);
}

#[test]
fn serde_cognate_entry() {
    let entry = lipi::lexicon::cognate::CognateEntry {
        language: Cow::Borrowed("en"),
        word: Cow::Borrowed("water"),
        ipa: Cow::Borrowed("ˈwɔːtər"),
    };
    let json = serde_json::to_string(&entry).unwrap();
    let back: lipi::lexicon::cognate::CognateEntry = serde_json::from_str(&json).unwrap();
    assert_eq!(entry, back);
}

#[test]
fn serde_etymology() {
    let etym = lipi::lexicon::cognate::Etymology {
        source_language: Cow::Borrowed("fr"),
        source_form: Cow::Borrowed("café"),
        borrowing_type: lipi::lexicon::cognate::BorrowingType::Loanword,
        period: Some(Cow::Borrowed("18th century")),
    };
    let json = serde_json::to_string(&etym).unwrap();
    let back: lipi::lexicon::cognate::Etymology = serde_json::from_str(&json).unwrap();
    assert_eq!(etym, back);
}

#[test]
fn serde_etymology_no_period() {
    let etym = lipi::lexicon::cognate::Etymology {
        source_language: Cow::Borrowed("la"),
        source_form: Cow::Borrowed("aqua"),
        borrowing_type: lipi::lexicon::cognate::BorrowingType::Inherited,
        period: None,
    };
    let json = serde_json::to_string(&etym).unwrap();
    let back: lipi::lexicon::cognate::Etymology = serde_json::from_str(&json).unwrap();
    assert_eq!(etym, back);
}

// ---------------------------------------------------------------------------
// Allophone types
// ---------------------------------------------------------------------------

#[test]
fn serde_allophone_rule_set() {
    let rules = lipi::phoneme::allophone::english_allophones();
    let json = serde_json::to_string(&rules).unwrap();
    let back: lipi::phoneme::allophone::AllophoneRuleSet = serde_json::from_str(&json).unwrap();
    assert_eq!(rules, back);
}

#[test]
fn serde_allophone_rule() {
    let rule = lipi::phoneme::allophone::AllophoneRule {
        phoneme: Cow::Borrowed("t"),
        allophone: Cow::Borrowed("ɾ"),
        environment: lipi::phoneme::allophone::Environment::Intervocalic,
        obligatory: false,
    };
    let json = serde_json::to_string(&rule).unwrap();
    let back: lipi::phoneme::allophone::AllophoneRule = serde_json::from_str(&json).unwrap();
    assert_eq!(rule, back);
}

// ---------------------------------------------------------------------------
// Syllable/Phonotactics types
// ---------------------------------------------------------------------------

#[test]
fn serde_phonotactics_all() {
    let profiles = [
        lipi::phoneme::syllable::english_phonotactics(),
        lipi::phoneme::syllable::sanskrit_phonotactics(),
        lipi::phoneme::syllable::japanese_phonotactics(),
    ];
    for p in &profiles {
        let json = serde_json::to_string(p).unwrap();
        let back: lipi::phoneme::syllable::Phonotactics = serde_json::from_str(&json).unwrap();
        assert_eq!(*p, back, "roundtrip failed for {}", p.language_code);
    }
}

#[test]
fn serde_syllable_template() {
    let tmpl = lipi::phoneme::syllable::SyllableTemplate {
        max_onset: 3,
        max_coda: 4,
        complex_nucleus: true,
        pattern: Cow::Borrowed("(C)(C)(C)V(C)(C)(C)(C)"),
    };
    let json = serde_json::to_string(&tmpl).unwrap();
    let back: lipi::phoneme::syllable::SyllableTemplate = serde_json::from_str(&json).unwrap();
    assert_eq!(tmpl, back);
}

// ---------------------------------------------------------------------------
// Dialect types
// ---------------------------------------------------------------------------

#[test]
fn serde_language_variety() {
    let rp = lipi::dialect::british_english();
    let json = serde_json::to_string(&rp).unwrap();
    let back: lipi::dialect::LanguageVariety = serde_json::from_str(&json).unwrap();
    assert_eq!(rp, back);
}

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

#[test]
fn error_debug_format() {
    // LipiError derives Debug — verify it doesn't panic
    let errors = vec![
        lipi::LipiError::UnknownLanguage("xx".into()),
        lipi::LipiError::UnknownScript("Xxxx".into()),
        lipi::LipiError::PhonemeNotInInventory {
            phoneme: "ʀ".into(),
            language: "en".into(),
        },
        lipi::LipiError::InvalidIpa("???".into()),
        lipi::LipiError::WordNotFound {
            word: "foo".into(),
            language: "en".into(),
        },
    ];
    for err in &errors {
        let debug = format!("{err:?}");
        assert!(!debug.is_empty());
        let display = format!("{err}");
        assert!(!display.is_empty());
    }
}
