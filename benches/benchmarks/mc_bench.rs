use criterion::{criterion_group, Criterion};
use mcdmrs::prom::multicriterion_flow;
use mcdmrs::prom::types::{Arr, Fl, Mat};
use rand::{distributions::Uniform, Rng};

pub fn mc_bench(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let range: Uniform<Fl> = Uniform::new(0.0, 1.0);

    let mut matrix_t: Mat = Vec::new();
    let m = 5;
    let n = 6000;
    for _ in 0..m {
        matrix_t.push((0..n).map(|_| 20.0 * rng.sample(&range) - 10.0).collect())
    }

    let pref = ["usual", "ushape", "vshape", "vshape2", "level"]
        .map(String::from)
        .to_vec();

    let q: Arr = vec![0.37, 0.95, 0.73, 0.60, 0.16];
    let p: Arr = vec![0.60, 0.71, 0.02, 0.97, 0.83];

    c.bench_function("n6000_m5", |b| {
        b.iter(|| multicriterion_flow(&matrix_t, &pref, &q, &p))
    });
}

criterion_group! {name=benches; config = Criterion::default().sample_size(15); targets=mc_bench}
