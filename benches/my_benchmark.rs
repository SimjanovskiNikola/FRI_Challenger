use criterion::{black_box, criterion_group, criterion_main, Criterion};
use engine::engine::board::perft::init_test_func;
use engine::engine::misc::const_utility::FEN_POS_FIVE;
use engine::engine::misc::const_utility::FEN_POS_SIX;
use engine::engine::misc::const_utility::FEN_START;

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
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
