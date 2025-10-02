use criterion::{Criterion, criterion_group, criterion_main};
use fri_check_mate::engine::misc::const_utility::FEN_POS_FIVE;
use fri_check_mate::engine::misc::const_utility::FEN_POS_FOUR;
use fri_check_mate::engine::misc::const_utility::FEN_POS_SIX;
use fri_check_mate::engine::misc::const_utility::FEN_START;
use fri_check_mate::engine::move_generator::perft::init_test_func;

use std::hint::black_box;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("FEN_START -> DEPTH: 1", |b| {
        b.iter(|| init_test_func(&FEN_START, black_box(1), false))
    });

    c.bench_function("FEN_START -> DEPTH: 4", |b| {
        b.iter(|| init_test_func(&FEN_START, black_box(4), false))
    });

    c.bench_function("FEN_POS_FIVE -> DEPTH: 3", |b| {
        b.iter(|| init_test_func(&FEN_POS_FIVE, black_box(3), false))
    });

    c.bench_function("FEN_POS_SIX -> DEPTH: 3", |b| {
        b.iter(|| init_test_func(&FEN_POS_SIX, black_box(3), false))
    });

    c.bench_function("FEN_POS_FOUR -> DEPTH: 5", |b| {
        b.iter(|| black_box(init_test_func(&FEN_POS_FOUR, 5, false)))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = criterion_benchmark
}
criterion_main!(benches);
