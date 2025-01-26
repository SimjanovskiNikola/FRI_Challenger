use criterion::{black_box, criterion_group, criterion_main, Criterion};

use engine::engine::{
    move_generation::perft::init_test_func, shared::helper_func::const_utility::FEN_START,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("init test func", |b| {
        b.iter(|| init_test_func(&FEN_START, black_box(1), false))
    });

    c.bench_function("init test func", |b| {
        b.iter(|| init_test_func(&FEN_START, black_box(4), false))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
