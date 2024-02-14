use mcdmrs::prom::utils;
use std::time::Instant;

fn bench() {
    let mut p: mcdmrs::prom::Prom = utils::generate_prom(6000, 24);

    let mut timings: Vec<f64> = Vec::new();

    for _ in 0..2 {
        let now: Instant = Instant::now();
        _ = p.compute_multicriterion_flow();
        timings.push(now.elapsed().as_secs_f64());
    }

    let s: f64 = timings.iter().sum::<f64>() * 1000.0;
    let avg: f64 = s / (timings.len() as f64);

    println!("avg time (ms):   {:.2}", avg);
    println!("total time (s):  {:.2}", s / 1000.);
}

fn main() {
    // rayon::ThreadPoolBuilder::new()
    //     .num_threads(1)
    //     .build_global()
    //     .unwrap();
    bench()
}
