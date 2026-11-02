use std::hint::black_box;

use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use ukrainian_tn::uktextnorm::{InputTolerance, NormalizeOptions};
use ukrainian_tn::{rozpodil, uktextnorm};

const MIXED: &str = "м. Київ, вул. Хрещатик, 1. Зустріч 01.05.2024 о 10:30. Вартість 1250 грн, вага 5 кг, температура +20 °C. Google і Spotify працюють.";
const ASR: &str = "увімкни вай фай та spotifay, сума педеве 1250 грн";

fn text_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("normalize");
    let asr_options =
        NormalizeOptions { input_tolerance: InputTolerance::Asr, ..Default::default() };
    // Initialize lazy regexes and lexicons before measuring steady-state calls.
    let _ = uktextnorm::normalize(MIXED);
    let _ = uktextnorm::normalize_with(ASR, &asr_options);
    group.throughput(Throughput::Bytes(MIXED.len() as u64));
    group.bench_function("mixed", |b| b.iter(|| uktextnorm::normalize(black_box(MIXED))));
    group.throughput(Throughput::Bytes(ASR.len() as u64));
    group.bench_function("asr", |b| {
        b.iter(|| uktextnorm::normalize_with(black_box(ASR), black_box(&asr_options)));
    });
    group.finish();

    let paragraph = MIXED.repeat(32);
    let mut group = c.benchmark_group("segmentation");
    group.throughput(Throughput::Bytes(paragraph.len() as u64));
    group.bench_function("tokenize", |b| b.iter(|| rozpodil::tokenize(black_box(&paragraph))));
    group.bench_function("sentences", |b| {
        b.iter(|| rozpodil::split_sentences(black_box(&paragraph)));
    });
    group.finish();

    c.bench_function("number_to_words", |b| {
        b.iter(|| uktextnorm::number_to_words(black_box(123_456_789)));
    });
}

criterion_group!(benches, text_processing);
criterion_main!(benches);
