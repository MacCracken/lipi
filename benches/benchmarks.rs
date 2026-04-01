use criterion::{Criterion, black_box, criterion_group, criterion_main};

use lipi::phoneme;

// ---------------------------------------------------------------------------
// Phoneme inventories
// ---------------------------------------------------------------------------

fn bench_english_inventory(c: &mut Criterion) {
    c.bench_function("english_phoneme_inventory", |b| b.iter(phoneme::english));
}

fn bench_sanskrit_inventory(c: &mut Criterion) {
    c.bench_function("sanskrit_phoneme_inventory", |b| b.iter(phoneme::sanskrit));
}

fn bench_greek_inventory(c: &mut Criterion) {
    c.bench_function("greek_phoneme_inventory", |b| b.iter(phoneme::greek));
}

fn bench_phoneme_lookup(c: &mut Criterion) {
    let en = phoneme::english();
    c.bench_function("phoneme_lookup_ipa", |b| b.iter(|| en.find(black_box("ʃ"))));
}

// ---------------------------------------------------------------------------
// Registry
// ---------------------------------------------------------------------------

fn bench_registry_lookup(c: &mut Criterion) {
    c.bench_function("registry_phonemes_lookup", |b| {
        b.iter(|| lipi::registry::phonemes(black_box("sa")))
    });
}

fn bench_registry_all_codes_iter(c: &mut Criterion) {
    c.bench_function("registry_all_codes_iter", |b| {
        b.iter(|| {
            for code in lipi::registry::all_codes() {
                black_box(lipi::registry::info(code));
            }
        })
    });
}

// ---------------------------------------------------------------------------
// Script
// ---------------------------------------------------------------------------

fn bench_script_lookup(c: &mut Criterion) {
    c.bench_function("script_by_code_lookup", |b| {
        b.iter(|| lipi::script::by_code(black_box("Deva")))
    });
}

fn bench_script_contains_codepoint(c: &mut Criterion) {
    let deva = lipi::script::devanagari();
    c.bench_function("script_contains_codepoint", |b| {
        b.iter(|| deva.contains_codepoint(black_box(0x0915)))
    });
}

// ---------------------------------------------------------------------------
// Transliteration
// ---------------------------------------------------------------------------

fn bench_transliteration_devanagari(c: &mut Criterion) {
    let table = lipi::script::transliteration::devanagari_iast();
    c.bench_function("transliterate_devanagari_char", |b| {
        b.iter(|| table.transliterate_char(black_box("क")))
    });
}

fn bench_transliteration_greek_word(c: &mut Criterion) {
    let table = lipi::script::transliteration::greek_beta_code();
    c.bench_function("transliterate_greek_word", |b| {
        b.iter(|| table.transliterate(black_box("φιλοσοφια")))
    });
}

// ---------------------------------------------------------------------------
// Numerals
// ---------------------------------------------------------------------------

fn bench_numeral_value_of(c: &mut Criterion) {
    let sys = lipi::script::numerals::greek_isopsephy();
    c.bench_function("numeral_value_of_char", |b| {
        b.iter(|| sys.value_of(black_box("π")))
    });
}

fn bench_numeral_string_value(c: &mut Criterion) {
    let sys = lipi::script::numerals::greek_isopsephy();
    c.bench_function("numeral_string_value_word", |b| {
        b.iter(|| sys.string_value(black_box("θεος")))
    });
}

// ---------------------------------------------------------------------------
// Grammar
// ---------------------------------------------------------------------------

fn bench_grammar_lookup(c: &mut Criterion) {
    c.bench_function("grammar_by_code_lookup", |b| {
        b.iter(|| lipi::grammar::by_code(black_box("ru")))
    });
}

// ---------------------------------------------------------------------------
// Lexicon / Swadesh
// ---------------------------------------------------------------------------

fn bench_swadesh_lookup(c: &mut Criterion) {
    c.bench_function("swadesh_by_code_lookup", |b| {
        b.iter(|| lipi::lexicon::swadesh::by_code(black_box("ja")))
    });
}

fn bench_lexicon_find(c: &mut Criterion) {
    let lex = lipi::lexicon::swadesh::by_code("es").unwrap();
    c.bench_function("lexicon_find_word", |b| {
        b.iter(|| lex.find(black_box("agua")))
    });
}

// ---------------------------------------------------------------------------
// Allophone
// ---------------------------------------------------------------------------

fn bench_allophone_realize(c: &mut Criterion) {
    let rules = lipi::phoneme::allophone::english_allophones();
    c.bench_function("allophone_realize", |b| {
        b.iter(|| {
            rules.realize(
                black_box("t"),
                black_box(&lipi::phoneme::allophone::Environment::Intervocalic),
            )
        })
    });
}

// ---------------------------------------------------------------------------
// Syllable / Phonotactics
// ---------------------------------------------------------------------------

fn bench_phonotactics_is_permitted(c: &mut Criterion) {
    let p = lipi::phoneme::syllable::english_phonotactics();
    c.bench_function("phonotactics_is_permitted", |b| {
        b.iter(|| {
            p.is_permitted(
                black_box("str"),
                black_box(lipi::phoneme::syllable::SyllablePosition::Onset),
            )
        })
    });
}

// ---------------------------------------------------------------------------
// Dialect
// ---------------------------------------------------------------------------

fn bench_dialect_apply(c: &mut Criterion) {
    let en = phoneme::english();
    let rp = lipi::dialect::british_english();
    c.bench_function("dialect_apply_overlay", |b| {
        b.iter(|| rp.apply(black_box(&en)))
    });
}

criterion_group!(
    benches,
    // Phoneme (4)
    bench_english_inventory,
    bench_sanskrit_inventory,
    bench_greek_inventory,
    bench_phoneme_lookup,
    // Registry (2)
    bench_registry_lookup,
    bench_registry_all_codes_iter,
    // Script (2)
    bench_script_lookup,
    bench_script_contains_codepoint,
    // Transliteration (2)
    bench_transliteration_devanagari,
    bench_transliteration_greek_word,
    // Numerals (2)
    bench_numeral_value_of,
    bench_numeral_string_value,
    // Grammar (1)
    bench_grammar_lookup,
    // Lexicon (2)
    bench_swadesh_lookup,
    bench_lexicon_find,
    // Allophone (1)
    bench_allophone_realize,
    // Syllable (1)
    bench_phonotactics_is_permitted,
    // Dialect (1)
    bench_dialect_apply,
);
criterion_main!(benches);
