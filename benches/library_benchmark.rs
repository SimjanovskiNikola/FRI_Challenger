use FRI_Challenger::engine::misc::const_utility::FEN_START;
use FRI_Challenger::engine::move_generator::perft::Stats;
use FRI_Challenger::engine::move_generator::perft::init_test_func;
use iai_callgrind::library_benchmark;
use iai_callgrind::library_benchmark_group;
use iai_callgrind::main;
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
