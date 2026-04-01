use criterion::{Criterion, black_box, criterion_group, criterion_main};

use lipi::phoneme;

fn bench_english_inventory(c: &mut Criterion) {
    c.bench_function("english_phoneme_inventory", |b| b.iter(phoneme::english));
}

fn bench_phoneme_lookup(c: &mut Criterion) {
    let en = phoneme::english();
    c.bench_function("phoneme_lookup_ipa", |b| b.iter(|| en.find(black_box("ʃ"))));
}

criterion_group!(benches, bench_english_inventory, bench_phoneme_lookup);
criterion_main!(benches);
