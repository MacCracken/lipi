//! Lexicon — Swadesh lists, cognates, and cross-language vocabulary.

fn main() {
    // --- Swadesh word for "water" across languages ---
    println!("=== 'water' across languages (Swadesh-25) ===\n");
    for code in lipi::lexicon::swadesh::all_codes() {
        let lex = lipi::lexicon::swadesh::by_code(code).unwrap();
        let water = lex.entries.iter().find(|e| e.gloss == "water").unwrap();
        let info = lipi::registry::info(code).unwrap();
        println!("  {:<12} {:<10} /{}/ ", info.name, water.word, water.ipa);
    }

    // --- Cognate tracking ---
    println!("\n=== Water Cognates (Proto-Indo-European) ===\n");
    let cog = lipi::lexicon::cognate::water_cognates();
    if let Some(proto) = &cog.proto_form {
        println!("  Proto-form: {proto}");
    }
    for entry in &cog.entries {
        let name = lipi::registry::info(&entry.language)
            .map(|i| i.name)
            .unwrap_or("?");
        println!("  {name:<12} {} /{}/", entry.word, entry.ipa);
    }

    // --- Lexicon queries ---
    println!("\n=== Spanish Swadesh-25 ===\n");
    let es = lipi::lexicon::swadesh::by_code("es").unwrap();
    for entry in &es.entries {
        println!(
            "  {:>2}. {:<12} /{:<12}/ {:?}",
            entry.swadesh_index.unwrap_or(0),
            entry.word,
            entry.ipa,
            entry.pos,
        );
    }
}
