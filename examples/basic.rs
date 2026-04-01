//! Basic usage of lipi — exploring phoneme inventories, scripts, and the registry.

use lipi::phoneme::{self, PhonemeKind};

fn main() {
    // --- Registry lookup ---
    println!("=== Registered Languages ===");
    for code in lipi::registry::all_codes() {
        let info = lipi::registry::info(code).unwrap();
        let inv = lipi::registry::phonemes(code).unwrap();
        let script = lipi::registry::primary_script(code).unwrap();
        println!(
            "  {} ({}) — {}C + {}V, script: {} ({:?})",
            info.name,
            info.code,
            inv.consonant_count(),
            inv.vowel_count(),
            script.name,
            script.script_type,
        );
    }
    println!();

    // --- English inventory ---
    let en = phoneme::english();
    println!("=== English Fricatives ===");
    for p in &en.phonemes {
        if let PhonemeKind::Consonant {
            manner: lipi::phoneme::Manner::Fricative,
            place,
            voiced,
            ..
        } = &p.kind
        {
            println!(
                "  /{}/  {:?}, {}",
                p.ipa,
                place,
                if *voiced { "voiced" } else { "voiceless" }
            );
        }
    }
    println!();

    // --- Sanskrit varga system ---
    let sa = phoneme::sanskrit();
    println!("=== Sanskrit Kavarga (Velar Stops) ===");
    for ipa in &["k", "kʰ", "ɡ", "ɡʰ", "ŋ"] {
        if sa.has(ipa) {
            println!("  /{ipa}/ ✓");
        }
    }
    println!();

    // --- Script lookup ---
    println!("=== Script: Devanagari ===");
    let deva = lipi::script::by_code("Deva").unwrap();
    println!("  Type: {:?}", deva.script_type);
    println!("  Direction: {:?}", deva.direction);
    println!("  Contains U+0915 (क): {}", deva.contains_codepoint(0x0915));

    // --- Builder pattern ---
    println!();
    println!("=== Custom Inventory (Builder) ===");
    let custom = phoneme::PhonemeInventoryBuilder::new("xx", "Example Language")
        .stress(phoneme::StressPattern::Fixed)
        .consonant(
            "p",
            phoneme::Manner::Plosive,
            phoneme::Place::Bilabial,
            false,
        )
        .consonant(
            "t",
            phoneme::Manner::Plosive,
            phoneme::Place::Alveolar,
            false,
        )
        .vowel(
            "a",
            phoneme::Height::Open,
            phoneme::Backness::Central,
            false,
        )
        .vowel("i", phoneme::Height::Close, phoneme::Backness::Front, false)
        .build();
    println!(
        "  {} — {}C + {}V, stress: {:?}",
        custom.language_name,
        custom.consonant_count(),
        custom.vowel_count(),
        custom.stress
    );
}
