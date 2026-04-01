use criterion::{Criterion, black_box, criterion_group, criterion_main};

use lipi::phoneme;

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

fn bench_registry_lookup(c: &mut Criterion) {
    c.bench_function("registry_phonemes_lookup", |b| {
        b.iter(|| lipi::registry::phonemes(black_box("sa")))
    });
}

fn bench_script_lookup(c: &mut Criterion) {
    c.bench_function("script_by_code_lookup", |b| {
        b.iter(|| lipi::script::by_code(black_box("Deva")))
    });
}

criterion_group!(
    benches,
    bench_english_inventory,
    bench_sanskrit_inventory,
    bench_greek_inventory,
    bench_phoneme_lookup,
    bench_registry_lookup,
    bench_script_lookup,
);
criterion_main!(benches);
