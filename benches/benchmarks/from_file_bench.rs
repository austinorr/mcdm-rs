use criterion::{criterion_group, Criterion};
use mcdmrs::prom::{
    interop,
    types::{Arr, Fl},
    Prom,
};

use polars::prelude::DataFrame;
use rand::{distributions::Uniform, thread_rng, Rng};

fn from_file_long(p: &mut Prom) -> Vec<Fl> {
    p.compute_prom_ii().expect("failed to compute prom.");
    p.mc_flow = None;
    p.prom_ii.as_ref().unwrap().score.to_vec()
}

fn reweight(p: &mut Prom, weights: Arr) -> Arr {
    p.re_weight(&weights).expect("failed to reweight prom.");
    p.prom_ii.as_ref().unwrap().score.to_vec()
}

pub fn from_file_bench(c: &mut Criterion) {
    const TESTFILE: &str = "./examples/data/alternatives_long.csv";
    const CRITFILE: &str = "./examples/data/criteria.csv";

    // * Import CSV
    let data_df = interop::polars::df_from_csv(TESTFILE).expect("failed to load data");
    let criteria_df: DataFrame =
        interop::polars::df_from_csv(CRITFILE).expect("failed to criteria");

    let p = interop::polars::prom_from_polars(&data_df, &criteria_df).unwrap();
    c.bench_function("from_file_long", move |b| {
        b.iter(|| from_file_long(&mut p.clone()))
    });
}

pub fn reweight_bench(c: &mut Criterion) {
    let mut rng = thread_rng();
    let range: Uniform<Fl> = Uniform::new(0.0, 1.0);

    const TESTFILE: &str = "./examples/data/alternatives_long.csv";
    const CRITFILE: &str = "./examples/data/criteria.csv";

    // * Import CSV
    let data_df = interop::polars::df_from_csv(TESTFILE).expect("failed to load data");
    let criteria_df: DataFrame =
        interop::polars::df_from_csv(CRITFILE).expect("failed to criteria");

    let p = interop::polars::prom_from_polars(&data_df, &criteria_df).unwrap();
    let weights = (0..p.criteria.weight.len())
        .map(|_| rng.sample(range))
        .collect::<Vec<Fl>>();

    c.bench_function("reweight", |b| {
        b.iter(|| reweight(&mut p.clone(), weights.clone()))
    });
}

criterion_group! {name=benches; config = Criterion::default().sample_size(15); targets=from_file_bench, reweight_bench}
