use engine::engine::{
    move_generation::perft::{init_test_func, Stats},
    shared::helper_func::const_utility::FEN_START,
};
use iai_callgrind::{main, library_benchmark_group, library_benchmark};
use std::hint::black_box;

fn perft_testing(is_long: bool) -> Stats {
    match is_long {
        true => init_test_func(&FEN_START, 3, false),
        false => init_test_func(&FEN_START, 6, false),
    }
}

#[library_benchmark]
#[bench::short(false)]
#[bench::long(true)]
fn bench_fn(is_long: bool) -> Stats {
    black_box(perft_testing(is_long))
}

library_benchmark_group!(name = bench_group; benchmarks = bench_fn);
main!(library_benchmark_groups = bench_group);
