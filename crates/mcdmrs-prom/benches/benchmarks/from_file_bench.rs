use criterion::{criterion_group, Criterion};
use mcdmrs_prom::{df_from_csv, types::Fl, FromPolars, Prom};
use ndarray::{Array1, ArrayView1};
use polars::prelude::DataFrame;
use rand::{distributions::Uniform, thread_rng, Rng};

const TESTFILE: &str = "../../examples/data/alternatives_long.csv";
const CRITFILE: &str = "../../examples/data/criteria.csv";

fn setup_prom() -> Prom {
    // * Import CSV
    let data_df = df_from_csv(TESTFILE).expect("failed to load data");
    let criteria_df: DataFrame = df_from_csv(CRITFILE).expect("failed to criteria");

    Prom::from_polars(&data_df, &criteria_df).unwrap()
}

fn from_file_long(p: &mut Prom) -> Vec<Fl> {
    p.compute_prom_ii().expect("failed to compute prom.");
    p.mc_flow = None;
    p.prom_ii.as_ref().unwrap().score.to_vec()
}

fn reweight(p: &mut Prom, weights: ArrayView1<Fl>) -> Vec<Fl> {
    p.re_weight(weights).expect("failed to reweight prom.");
    p.prom_ii.as_ref().unwrap().score.to_vec()
}

pub fn from_file_bench(c: &mut Criterion) {
    let mut p = setup_prom();

    c.bench_function("from_file_long", move |b| b.iter(|| from_file_long(&mut p)));
}

pub fn reweight_bench(c: &mut Criterion) {
    let mut rng = thread_rng();
    let range: Uniform<Fl> = Uniform::new(0.0, 1.0);

    let mut p = setup_prom();
    // let mut p = mcdmrs_prom::utils::generate_prom(50000, 20).unwrap(); // this bench is so fast i tested it with a much larger input
    let _ = p.compute_prom_ii();

    let wa = Array1::<Fl>::from_iter((0..p.criteria.weight.len()).map(|_| rng.sample(range)));
    let weights = wa.view();

    c.bench_function("reweight", |b| b.iter(|| reweight(&mut p, weights)));
}

criterion_group! {name=benches; config = Criterion::default().sample_size(15); targets=from_file_bench, reweight_bench}
