//! Basic usage of lipi — exploring the English phoneme inventory.

use lipi::phoneme::{self, PhonemeKind};

fn main() {
    let en = phoneme::english();

    println!("Language: {} ({})", en.language_name, en.language_code);
    println!("Stress: {:?}", en.stress);
    println!("Consonants: {}", en.consonant_count());
    println!("Vowels: {}", en.vowel_count());
    println!();

    // List all fricatives
    println!("Fricatives:");
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

    // Look up a specific phoneme
    if let Some(sh) = en.find("ʃ") {
        println!("Found /ʃ/: {:?}", sh.kind);
    }

    // Check for phonemes
    println!("/θ/ in English: {}", en.has("θ")); // true
    println!("/ʀ/ in English: {}", en.has("ʀ")); // false (no uvular trill)
}
