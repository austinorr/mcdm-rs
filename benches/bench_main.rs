use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    benchmarks::mc_bench::benches,
    benchmarks::uc_bench::benches,
}
