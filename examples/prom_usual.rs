use mcdmrs::prom::{utils, Prom};
use std::time::Instant;

fn bench(mut p: Prom, iters: usize) {
    let mut timings: Vec<f64> = Vec::new();

    for _ in 0..iters {
        let now: Instant = Instant::now();
        _ = p.compute_multicriterion_flow();
        timings.push(now.elapsed().as_secs_f64());
    }

    let s: f64 = timings.iter().sum::<f64>() * 1000.0;
    let avg: f64 = s / (timings.len() as f64);

    println!("avg time [{:?} iterations] (ms):  {:.2}", iters, avg);
    println!("total time (s):  {:.2}", s / 1000.);
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    let n: usize;
    match args.get(1) {
        Some(s) => n = s.parse().unwrap(),
        None => n = 25,
    }
    let m: usize;
    match args.get(2) {
        Some(s) => m = s.parse().unwrap(),
        None => m = 8,
    }
    let iters: usize;
    match args.get(3) {
        Some(s) => iters = s.parse().unwrap(),
        None => iters = 10,
    }

    println!("Running Prom with dimensions {}x{}.", n, m);
    let p: mcdmrs::prom::Prom = utils::generate_prom(n, m);
    bench(p, iters);
}
