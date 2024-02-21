use criterion::{criterion_group, Criterion};
use mcdmrs::prom::types::{Arr, Fl};
use mcdmrs::prom::unicriterion_flow::unicriterion_flow_usual;
use rand::{distributions::Uniform, Rng};

pub fn uc_bench(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let range: Uniform<Fl> = Uniform::new(0.0, 1.0);

    let n = 8000;
    let array: Arr = (0..n).map(|_| 20.0 * rng.sample(range)).collect();

    let q: Fl = 0.;
    let p: Fl = 0.;

    let mut plus: Vec<Fl> = vec![0.0; array.len()];
    let mut minus: Vec<Fl> = vec![0.0; array.len()];

    c.bench_function("n8000", |b| {
        b.iter(|| unicriterion_flow_usual(&array, &mut plus, &mut minus, &q, &p))
    });
}

criterion_group! {name=benches; config = Criterion::default().sample_size(15); targets=uc_bench}
