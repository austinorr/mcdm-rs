#[cfg(all(feature = "io", feature = "cli"))]
fn prom_from_file() {
    use mcdmrs::prom::{interop, Prom};
    use polars::prelude::{DataFrame, NamedFrom, Series};

    const TESTFILE: &str = "./examples/data/alternatives.csv";
    const CRITFILE: &str = "./examples/data/criteria.csv";

    pub fn configure_the_environment() {
        use std::env;
        env::set_var("POLARS_FMT_TABLE_ROUNDED_CORNERS", "1"); // apply rounded corners to UTF8-styled tables.
        env::set_var("POLARS_FMT_MAX_COLS", "20"); // maximum number of columns shown when formatting DataFrames.
        env::set_var("POLARS_FMT_MAX_ROWS", "50"); // maximum number of rows shown when formatting DataFrames.
        env::set_var("POLARS_FMT_STR_LEN", "50"); // maximum number of characters printed per string value.
    }

    // * Import CSV
    let mut data_df = interop::polars::df_from_csv(TESTFILE).expect("failed to load data");
    let criteria_df: DataFrame =
        interop::polars::df_from_csv(CRITFILE).expect("failed to criteria");

    let mut p: Prom = interop::polars::prom_from_polars(&data_df, &criteria_df).unwrap();
    p.compute_prom_ii().expect("failed to compute prom.");

    let score = Series::new(
        "score",
        p.prom_ii.as_ref().unwrap().normalized_score.to_vec(),
    );
    let normalized_score = Series::new(
        "normalized_score",
        p.prom_ii.as_ref().unwrap().normalized_score.to_vec(),
    );
    data_df.with_column(score).unwrap();
    data_df.with_column(normalized_score).unwrap();

    configure_the_environment();
    println!("Scoring Criteria {:#?}", criteria_df);
    println!("Data with Prom II Scores {:#?}", data_df);
}

fn main() {
    #[cfg(all(feature = "io", feature = "cli"))]
    prom_from_file()
}
