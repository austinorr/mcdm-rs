use mcdmrs_prom::{
    types::{Fl, Result},
    utils, Prom,
};
use ndarray::Array1;
use rand::{distributions::Uniform, Rng};
use std::time::Instant;

fn bench(mut p: Prom, iters: usize) {
    let mut rng = rand::thread_rng();
    let range: Uniform<Fl> = Uniform::new(0.0, 20.0);

    let mut timings: Vec<f64> = Vec::new();
    let mut pr_timings: Vec<f64> = Vec::new();
    let mut rw_timings: Vec<f64> = Vec::new();

    for _ in 0..iters {
        let now: Instant = Instant::now();
        _ = p.compute_multicriterion_flow();
        timings.push(now.elapsed().as_secs_f64());

        let now: Instant = Instant::now();
        _ = p.compute_prom_ii();
        pr_timings.push(now.elapsed().as_secs_f64());

        let weight =
            Array1::<Fl>::from_iter((0..p.criteria.weight.len()).map(|_| rng.sample(range)));

        let now: Instant = Instant::now();
        _ = p.re_weight(weight.view());
        rw_timings.push(now.elapsed().as_secs_f64());

        p.mc_flow = None;
    }

    let s: f64 = timings.iter().sum::<f64>() * 1000.0;
    let avg: f64 = s / (timings.len() as f64);

    let pr_s: f64 = pr_timings.iter().sum::<f64>() * 1000.0;
    let pr_avg: f64 = pr_s / (pr_timings.len() as f64);

    let rw_s: f64 = rw_timings.iter().sum::<f64>() * 1000.0;
    let rw_avg: f64 = rw_s / (rw_timings.len() as f64);

    println!(
        "avg mc flow time [{:?} iterations] (ms):  {:.2}",
        iters, avg
    );
    println!(
        "avg Prom II time [{:?} iterations] (ms):  {:.2}",
        iters, pr_avg
    );
    println!(
        "avg reweight time [{:?} iterations] (ms):  {:.2}",
        iters, rw_avg
    );
    println!("total time (s):  {:.2}", (s + rw_s) / 1000.);
}

fn main() -> Result<()> {
    use std::env;
    let args: Vec<String> = env::args().collect();

    let n: usize = match args.get(1) {
        Some(s) => s.parse().unwrap(),
        None => 25,
    };
    let m: usize = match args.get(2) {
        Some(s) => s.parse().unwrap(),
        None => 8,
    };
    let iters: usize = match args.get(3) {
        Some(s) => s.parse().unwrap(),
        None => 10,
    };

    println!("Running Prom with dimensions {}x{}.", n, m);
    let p: Prom = utils::generate_prom(n, m)?;
    bench(p, iters);

    Ok(())
}
