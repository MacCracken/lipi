//! Invariant tests — linguistic consistency properties that must always hold.
//!
//! These tests verify structural invariants across the entire dataset:
//! registry alignment, phoneme uniqueness, index completeness, and
//! cross-module consistency.

use std::collections::HashSet;

// ---------------------------------------------------------------------------
// Registry ↔ phoneme alignment
// ---------------------------------------------------------------------------

#[test]
fn every_registered_language_has_phonemes() {
    for lang in lipi::registry::REGISTERED {
        assert!(
            lipi::registry::phonemes(lang.code).is_some(),
            "registered language {} has no phoneme inventory",
            lang.code
        );
    }
}

#[test]
fn all_codes_matches_registered_count() {
    let codes = lipi::registry::all_codes();
    assert_eq!(
        codes.len(),
        lipi::registry::REGISTERED.len(),
        "all_codes() length != REGISTERED length"
    );
}

#[test]
fn all_codes_are_unique() {
    let codes = lipi::registry::all_codes();
    let unique: HashSet<_> = codes.iter().collect();
    assert_eq!(codes.len(), unique.len(), "duplicate language codes found");
}

#[test]
fn registered_codes_match_all_codes() {
    let codes: HashSet<_> = lipi::registry::all_codes().iter().copied().collect();
    for lang in lipi::registry::REGISTERED {
        assert!(
            codes.contains(lang.code),
            "REGISTERED has {} but all_codes() does not",
            lang.code
        );
    }
}

// ---------------------------------------------------------------------------
// Phoneme inventory invariants
// ---------------------------------------------------------------------------

#[test]
fn consonant_plus_vowel_equals_total_for_all_languages() {
    for code in lipi::registry::all_codes() {
        let inv = lipi::registry::phonemes(code).unwrap();
        assert_eq!(
            inv.consonant_count() + inv.vowel_count(),
            inv.phonemes.len(),
            "consonant + vowel != total for {}",
            code
        );
    }
}

#[test]
fn no_duplicate_ipa_symbols_in_any_inventory() {
    for code in lipi::registry::all_codes() {
        let inv = lipi::registry::phonemes(code).unwrap();
        let mut seen = HashSet::new();
        for p in &inv.phonemes {
            assert!(
                seen.insert(&p.ipa),
                "duplicate IPA /{}/  in {} inventory",
                p.ipa,
                code
            );
        }
    }
}

#[test]
fn every_inventory_has_at_least_one_consonant_and_vowel() {
    for code in lipi::registry::all_codes() {
        let inv = lipi::registry::phonemes(code).unwrap();
        assert!(inv.consonant_count() > 0, "{} has zero consonants", code);
        assert!(inv.vowel_count() > 0, "{} has zero vowels", code);
    }
}

#[test]
fn language_code_in_inventory_matches_registry() {
    for code in lipi::registry::all_codes() {
        let inv = lipi::registry::phonemes(code).unwrap();
        assert_eq!(
            inv.language_code.as_ref(),
            *code,
            "inventory language_code {} != registry code {}",
            inv.language_code,
            code
        );
    }
}

#[test]
fn tonal_languages_have_tonal_stress_pattern() {
    for code in lipi::registry::all_codes() {
        let inv = lipi::registry::phonemes(code).unwrap();
        if inv.tones.is_some() {
            assert_eq!(
                inv.stress,
                lipi::phoneme::StressPattern::Tonal,
                "{} has tones but stress != Tonal",
                code
            );
        }
        if inv.stress == lipi::phoneme::StressPattern::Tonal {
            assert!(
                inv.tones.is_some(),
                "{} has Tonal stress but no tone list",
                code
            );
        }
    }
}

#[test]
fn no_empty_ipa_symbols() {
    for code in lipi::registry::all_codes() {
        let inv = lipi::registry::phonemes(code).unwrap();
        for p in &inv.phonemes {
            assert!(!p.ipa.is_empty(), "empty IPA symbol in {} inventory", code);
        }
    }
}

// ---------------------------------------------------------------------------
// Script invariants
// ---------------------------------------------------------------------------

#[test]
fn all_script_codes_are_unique() {
    let codes = lipi::script::all_codes();
    let unique: HashSet<_> = codes.iter().collect();
    assert_eq!(codes.len(), unique.len(), "duplicate script codes");
}

#[test]
fn every_script_code_resolves() {
    for code in lipi::script::all_codes() {
        let script = lipi::script::by_code(code);
        assert!(script.is_some(), "by_code({code}) returned None");
        assert_eq!(
            script.unwrap().code.as_ref(),
            *code,
            "script code mismatch for {code}"
        );
    }
}

#[test]
fn every_script_has_at_least_one_unicode_range() {
    for code in lipi::script::all_codes() {
        let script = lipi::script::by_code(code).unwrap();
        assert!(
            !script.unicode_ranges.is_empty(),
            "script {} has no unicode ranges",
            code
        );
    }
}

#[test]
fn unicode_ranges_are_well_formed() {
    for code in lipi::script::all_codes() {
        let script = lipi::script::by_code(code).unwrap();
        for (lo, hi) in &script.unicode_ranges {
            assert!(
                lo <= hi,
                "script {} has inverted range: {:#X}-{:#X}",
                code,
                lo,
                hi
            );
        }
    }
}

#[test]
fn registry_script_codes_resolve() {
    for lang in lipi::registry::REGISTERED {
        for sc in lang.script_codes {
            // Some scripts may not be registered yet (Thai, Beng, etc.)
            // but the ones that ARE registered must resolve correctly
            if let Some(script) = lipi::script::by_code(sc) {
                assert_eq!(script.code.as_ref(), *sc);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Grammar invariants
// ---------------------------------------------------------------------------

#[test]
fn all_grammar_codes_are_unique() {
    let codes = lipi::grammar::all_codes();
    let unique: HashSet<_> = codes.iter().collect();
    assert_eq!(codes.len(), unique.len(), "duplicate grammar codes");
}

#[test]
fn every_grammar_code_resolves() {
    for code in lipi::grammar::all_codes() {
        let g = lipi::grammar::by_code(code);
        assert!(g.is_some(), "grammar::by_code({code}) returned None");
        assert_eq!(g.unwrap().language_code.as_ref(), *code);
    }
}

#[test]
fn gender_consistency() {
    for code in lipi::grammar::all_codes() {
        let g = lipi::grammar::by_code(code).unwrap();
        if g.has_gender {
            assert!(g.gender_count > 0, "{code}: has_gender but gender_count=0");
        } else {
            assert_eq!(
                g.gender_count, 0,
                "{code}: !has_gender but gender_count={}",
                g.gender_count
            );
        }
    }
}

#[test]
fn grammar_languages_are_registered() {
    for code in lipi::grammar::all_codes() {
        assert!(
            lipi::registry::info(code).is_some(),
            "grammar profile for {} but not registered in registry",
            code
        );
    }
}

// ---------------------------------------------------------------------------
// Swadesh invariants
// ---------------------------------------------------------------------------

#[test]
fn all_swadesh_codes_are_unique() {
    let codes = lipi::lexicon::swadesh::all_codes();
    let unique: HashSet<_> = codes.iter().collect();
    assert_eq!(codes.len(), unique.len(), "duplicate swadesh codes");
}

#[test]
fn every_swadesh_list_has_25_entries() {
    for code in lipi::lexicon::swadesh::all_codes() {
        let lex = lipi::lexicon::swadesh::by_code(code).unwrap();
        assert_eq!(
            lex.entries.len(),
            25,
            "{code} Swadesh list has {} entries, expected 25",
            lex.entries.len()
        );
    }
}

#[test]
fn swadesh_indices_are_1_through_25() {
    for code in lipi::lexicon::swadesh::all_codes() {
        let lex = lipi::lexicon::swadesh::by_code(code).unwrap();
        let indices: HashSet<u16> = lex.entries.iter().filter_map(|e| e.swadesh_index).collect();
        for i in 1..=25 {
            assert!(
                indices.contains(&i),
                "{code} Swadesh list missing index {i}"
            );
        }
    }
}

#[test]
fn swadesh_all_entries_have_non_empty_fields() {
    for code in lipi::lexicon::swadesh::all_codes() {
        let lex = lipi::lexicon::swadesh::by_code(code).unwrap();
        for entry in &lex.entries {
            assert!(!entry.word.is_empty(), "{code}: empty word");
            assert!(
                !entry.ipa.is_empty(),
                "{code}: empty IPA for {}",
                entry.word
            );
            assert!(
                !entry.gloss.is_empty(),
                "{code}: empty gloss for {}",
                entry.word
            );
        }
    }
}

#[test]
fn swadesh_languages_are_registered() {
    for code in lipi::lexicon::swadesh::all_codes() {
        assert!(
            lipi::registry::info(code).is_some(),
            "Swadesh data for {} but not registered in registry",
            code
        );
    }
}

// ---------------------------------------------------------------------------
// Transliteration invariants
// ---------------------------------------------------------------------------

#[test]
fn transliteration_forward_map_not_empty() {
    let tables = [
        lipi::script::transliteration::devanagari_iast(),
        lipi::script::transliteration::greek_beta_code(),
    ];
    for table in &tables {
        assert!(
            !table.forward.is_empty(),
            "transliteration table {} has empty forward map",
            table.scheme
        );
    }
}

#[test]
fn transliteration_every_mapping_has_source() {
    let tables = [
        lipi::script::transliteration::devanagari_iast(),
        lipi::script::transliteration::greek_beta_code(),
    ];
    for table in &tables {
        for (src, _) in &table.forward {
            assert!(
                !src.is_empty(),
                "empty source in {} transliteration table",
                table.scheme
            );
        }
    }
}

#[test]
fn transliteration_roundtrip_individual_chars() {
    // Every source char should transliterate to its expected target
    let table = lipi::script::transliteration::greek_beta_code();
    for (src, tgt) in &table.forward {
        let result = table.transliterate(src);
        assert_eq!(
            result,
            tgt.as_ref(),
            "transliteration mismatch: {} → {} (expected {})",
            src,
            result,
            tgt
        );
    }
}

// ---------------------------------------------------------------------------
// Numeral invariants
// ---------------------------------------------------------------------------

#[test]
fn numeral_mappings_not_empty() {
    let systems = [
        lipi::script::numerals::devanagari_digits(),
        lipi::script::numerals::greek_isopsephy(),
        lipi::script::numerals::babylonian_sexagesimal(),
        lipi::script::numerals::egyptian_hieroglyphic(),
        lipi::script::numerals::chinese_rod_numerals(),
    ];
    for sys in &systems {
        assert!(
            !sys.mappings.is_empty(),
            "numeral system {} has empty mappings",
            sys.name
        );
    }
}

#[test]
fn numeral_no_duplicate_characters() {
    let systems = [
        lipi::script::numerals::devanagari_digits(),
        lipi::script::numerals::greek_isopsephy(),
        lipi::script::numerals::babylonian_sexagesimal(),
        lipi::script::numerals::egyptian_hieroglyphic(),
        lipi::script::numerals::chinese_rod_numerals(),
    ];
    for sys in &systems {
        let mut seen = HashSet::new();
        for m in &sys.mappings {
            assert!(
                seen.insert(&m.character),
                "duplicate character {} in {} numeral system",
                m.character,
                sys.name
            );
        }
    }
}

#[test]
fn decimal_systems_have_digits_0_through_9() {
    let decimal_systems = [
        lipi::script::numerals::devanagari_digits(),
        lipi::script::numerals::chinese_rod_numerals(),
    ];
    for sys in &decimal_systems {
        if sys.kind == lipi::script::numerals::NumeralSystemKind::Decimal {
            let values: HashSet<u32> = sys.mappings.iter().map(|m| m.value).collect();
            // Devanagari has 0-9, Chinese rod has 1-9
            if values.contains(&0) {
                for d in 0..=9 {
                    assert!(values.contains(&d), "{} missing digit {d}", sys.name);
                }
            } else {
                for d in 1..=9 {
                    assert!(values.contains(&d), "{} missing digit {d}", sys.name);
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Cross-module invariants
// ---------------------------------------------------------------------------

#[test]
fn cognate_languages_are_registered() {
    let cog = lipi::lexicon::cognate::water_cognates();
    for entry in &cog.entries {
        // All cognate languages should be in the registry
        assert!(
            lipi::registry::info(&entry.language).is_some(),
            "cognate language {} not registered",
            entry.language
        );
    }
}

#[test]
fn dialect_parent_is_registered() {
    let rp = lipi::dialect::british_english();
    assert!(
        lipi::registry::info(&rp.parent).is_some(),
        "dialect parent {} not registered",
        rp.parent
    );
}

#[test]
fn dialect_overlay_preserves_phoneme_count_direction() {
    // Applying a variety should add/remove the expected number of phonemes
    let en = lipi::phoneme::english();
    let rp = lipi::dialect::british_english();
    let rp_inv = rp.apply(&en);

    let expected_delta = rp.added_phonemes.len() as i32 - rp.removed_phonemes.len() as i32;
    let actual_delta = rp_inv.phonemes.len() as i32 - en.phonemes.len() as i32;
    assert_eq!(
        actual_delta, expected_delta,
        "dialect overlay phoneme delta mismatch"
    );
}
