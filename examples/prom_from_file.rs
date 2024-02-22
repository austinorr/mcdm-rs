use mcdmrs::prom::types::Result;

#[cfg(all(feature = "io", feature = "cli"))]
fn prom_from_file() -> Result<()> {
    use mcdmrs::prom::{df_from_csv, FromPolars, Prom};
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
    let mut data_df = df_from_csv(TESTFILE)?;
    let criteria_df: DataFrame = df_from_csv(CRITFILE)?;

    let mut p: Prom = Prom::from_polars(&data_df, &criteria_df)?;
    p.compute_prom_ii()?;

    let pii = p.prom_ii.ok_or("Could not calculate Prom II")?;

    let score = Series::new("score", pii.score.to_vec());
    let normalized_score = Series::new("normalized_score", pii.normalized_score.to_vec());
    data_df.with_column(score)?;
    data_df.with_column(normalized_score)?;

    configure_the_environment();
    println!("Scoring Criteria {:#?}", criteria_df);
    println!("Data with Prom II Scores {:#?}", data_df);

    Ok(())
}

fn main() {
    #[cfg(all(feature = "io", feature = "cli"))]
    prom_from_file().unwrap()
}
