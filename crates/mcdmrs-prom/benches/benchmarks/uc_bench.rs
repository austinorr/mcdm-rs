use criterion::{criterion_group, Criterion};
use mcdmrs_prom::types::Fl;
use mcdmrs_prom::unicriterion_flow::unicriterion_flow_usual;
use ndarray::Array1;
use rand::{distributions::Uniform, Rng};

pub fn uc_bench(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let range: Uniform<Fl> = Uniform::new(0.0, 1.0);

    let n = 8000;
    let array = Array1::<Fl>::from_iter((0..n).map(|_| 20.0 * rng.sample(range) - 10.0));

    let q: Fl = 0.;
    let p: Fl = 0.;

    let (mut plus, mut minus) = (Array1::<Fl>::zeros(n), Array1::<Fl>::zeros(n));

    c.bench_function("n8000", |b| {
        b.iter(|| unicriterion_flow_usual(array.view(), plus.view_mut(), minus.view_mut(), &q, &p))
    });
}

criterion_group! {name=benches; config = Criterion::default().sample_size(15); targets=uc_bench}
