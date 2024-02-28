use criterion::{criterion_group, Criterion};
use mcdmrs_prom::multicriterion_flow;
use mcdmrs_prom::types::Fl;
use ndarray::{array, Array};
use rand::{distributions::Uniform, Rng};

pub fn mc_bench(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let range: Uniform<Fl> = Uniform::new(0.0, 1.0);
    let m = 5;
    let n = 6000;

    let mat = Array::from_iter((0..(n * m)).map(|_| 20.0 * rng.sample(range) - 10.0))
        .into_shape((m, n))
        .unwrap();

    let pref =
        Array::from_iter(["usual", "ushape", "vshape", "vshape2", "level"].map(String::from));

    let q = array![0.37, 0.95, 0.73, 0.60, 0.16];
    let p = array![0.60, 0.71, 0.02, 0.97, 0.83];

    c.bench_function("n6000_m5", |b| {
        b.iter(|| multicriterion_flow(mat.view(), pref.view(), q.view(), p.view()))
    });
}

criterion_group! {name=benches; config = Criterion::default().sample_size(15); targets=mc_bench}
