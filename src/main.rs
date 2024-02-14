use promrs::{prom::prom::Prom, prom::types};
use rand::{distributions::Uniform, Rng};
use std::time::Instant;

fn generate_prom(n: usize, m: usize) -> Prom {
    let mut rng = rand::thread_rng();
    let range: Uniform<types::Fl> = Uniform::new(0.0, 20.0);

    let mut matrix_t: types::Mat = Vec::new();
    for _ in 0..m {
        matrix_t.push((0..n).map(|_| rng.sample(&range)).collect())
    }

    let len: usize = matrix_t.len();

    Prom::new(
        matrix_t,
        vec![1.; len],
        vec![1.; len],
        vec!["usual".to_string(); len],
        vec![0.; len],
        vec![0.; len],
    )
    .expect("unable to build with Prom::new")
}

fn bench() {
    let mut p = generate_prom(6000, 24);

    let mut timings: Vec<f64> = Vec::new();

    for _ in 0..7 {
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
