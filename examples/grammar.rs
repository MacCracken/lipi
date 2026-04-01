//! Grammar profiles — morphological typology, word order, case systems.

fn main() {
    println!("=== Grammar Profiles ===\n");

    for code in lipi::grammar::all_codes() {
        let g = lipi::grammar::by_code(code).unwrap();
        let info = lipi::registry::info(code).unwrap();
        println!("{} ({code})", info.name);
        println!("  Morphology: {:?}", g.morphology);
        println!("  Word order: {:?}", g.word_order);
        if g.case_count > 0 {
            println!("  Cases: {}", g.case_count);
        }
        if g.has_gender {
            println!("  Genders: {}", g.gender_count);
        }
        if g.has_dual {
            println!("  Has dual number");
        }
        if g.has_classifiers {
            println!("  Uses classifiers");
        }
        println!();
    }

    // Compare typologies
    println!("=== Typology Comparison ===\n");
    let isolating: Vec<_> = lipi::grammar::all_codes()
        .iter()
        .filter(|c| {
            lipi::grammar::by_code(c).unwrap().morphology == lipi::grammar::Morphology::Isolating
        })
        .copied()
        .collect();
    let agglutinative: Vec<_> = lipi::grammar::all_codes()
        .iter()
        .filter(|c| {
            lipi::grammar::by_code(c).unwrap().morphology
                == lipi::grammar::Morphology::Agglutinative
        })
        .copied()
        .collect();

    println!("  Isolating: {isolating:?}");
    println!("  Agglutinative: {agglutinative:?}");
}
