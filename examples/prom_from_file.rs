use polars::prelude::NamedFrom;

pub fn configure_the_environment() {
    use std::env;
    env::set_var("POLARS_FMT_TABLE_ROUNDED_CORNERS", "1"); // apply rounded corners to UTF8-styled tables.
    env::set_var("POLARS_FMT_MAX_COLS", "20"); // maximum number of columns shown when formatting DataFrames.
    env::set_var("POLARS_FMT_MAX_ROWS", "50"); // maximum number of rows shown when formatting DataFrames.
    env::set_var("POLARS_FMT_STR_LEN", "50"); // maximum number of characters printed per string value.
}

fn main() {
    use mcdmrs::prom::{interop, Prom};
    use polars::prelude::{DataFrame, Series};

    const TESTFILE: &str = "./examples/data/alternatives.csv";
    const CRITFILE: &str = "./examples/data/criteria.csv";

    // * Import CSV
    let mut data_df = interop::polars::df_from_csv(TESTFILE).expect("failed to load data");
    let criteria_df: DataFrame =
        interop::polars::df_from_csv(CRITFILE).expect("failed to criteria");

    let mut p: Prom = interop::polars::prom_from_polars(&data_df, &criteria_df).unwrap();
    _ = p.compute_prom_ii().expect("failed to compute prom.");

    let score = Series::new(
        "score",
        p.prom_ii.as_ref().unwrap().normalized_score.clone(),
    );
    let normalized_score = Series::new(
        "normalized_score",
        p.prom_ii.as_ref().unwrap().normalized_score.clone(),
    );
    data_df.with_column(score).unwrap();
    data_df.with_column(normalized_score).unwrap();

    configure_the_environment();
    println!("Scoring Criteria {:#?}", criteria_df);
    println!("Data with Prom II Scores {:#?}", data_df);
}
