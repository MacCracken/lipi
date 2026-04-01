//! Scripts — writing system metadata, transliteration, and numeral systems.

fn main() {
    // --- All registered scripts ---
    println!("=== Registered Scripts ===\n");
    for code in lipi::script::all_codes() {
        let s = lipi::script::by_code(code).unwrap();
        println!(
            "  {} ({}) — {:?}, {:?}, {:?}",
            s.name, s.code, s.script_type, s.direction, s.status
        );
        if let Some(att) = &s.attestation {
            println!("    Attestation: {att}");
        }
    }

    // --- Transliteration: Devanagari → IAST ---
    println!("\n=== Devanagari → IAST Transliteration ===\n");
    let table = lipi::script::transliteration::devanagari_iast();
    let samples = ["अ", "क", "ॐ"];
    for s in samples {
        println!("  {} → {}", s, table.transliterate(s));
    }

    // --- Transliteration: Greek → Beta Code ---
    println!("\n=== Greek → Beta Code ===\n");
    let table = lipi::script::transliteration::greek_beta_code();
    let words = ["λογος", "Αθηνα", "φιλοσοφια"];
    for w in words {
        println!("  {} → {}", w, table.transliterate(w));
    }

    // --- Greek isopsephy ---
    println!("\n=== Greek Isopsephy ===\n");
    let sys = lipi::script::numerals::greek_isopsephy();
    let words = ["θεος", "αω", "πι"];
    for w in words {
        if let Some(val) = sys.string_value(w) {
            println!("  {w} = {val}");
        }
    }

    // --- Devanagari digits ---
    println!("\n=== Devanagari Digits ===\n");
    let sys = lipi::script::numerals::devanagari_digits();
    for d in 0..=9 {
        if let Some(ch) = sys.char_for(d) {
            println!("  {d} → {ch}");
        }
    }

    // --- Unicode codepoint detection ---
    println!("\n=== Unicode Codepoint Detection ===\n");
    let test_codepoints = [
        (0x0041, "A"),
        (0x0915, "क"),
        (0x0627, "ﺍ"),
        (0x03B1, "α"),
        (0x4E00, "一"),
    ];
    for (cp, label) in test_codepoints {
        for code in lipi::script::all_codes() {
            let s = lipi::script::by_code(code).unwrap();
            if s.contains_codepoint(cp) {
                println!("  U+{cp:04X} ({label}) → {} ({})", s.name, s.code);
            }
        }
    }
}
