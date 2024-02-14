use criterion::{criterion_group, Criterion};
use mcdmrs::prom::multicriterion_flow;
use mcdmrs::prom::types::{Fl, Mat};
use rand::{distributions::Uniform, Rng};

pub fn mc_bench(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let range: Uniform<Fl> = Uniform::new(0.0, 1.0);

    let mut matrix_t: Mat = Vec::new();
    let m = 5;
    let n = 6000;
    for _ in 0..m {
        matrix_t.push((0..n).map(|_| 20.0 * rng.sample(&range)).collect())
    }

    let len: usize = matrix_t.len();

    let pref = ["usual", "ushape", "vshape", "vshape2", "level"]
        .map(String::from)
        .to_vec();

    let q: Vec<Fl> = vec![0.; len];
    let p: Vec<Fl> = vec![0.; len];

    c.bench_function("n6000_m5", |b| {
        b.iter(|| multicriterion_flow(&matrix_t, &pref, &q, &p))
    });
}

criterion_group! {name=benches; config = Criterion::default().sample_size(15); targets=mc_bench}
