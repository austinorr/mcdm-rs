#[cfg(all(feature = "io", feature = "cli"))]
fn run_cli() {
    extern crate clap;
    use clap::Parser;
    use mcdmrs::prom::{interop, Prom};
    extern crate polars;
    use polars::prelude::{DataFrame, NamedFrom, Series};
    use std::path::PathBuf;
    use std::time::Instant;

    fn configure_the_environment() {
        use std::env;

        env::set_var("POLARS_FMT_TABLE_ROUNDED_CORNERS", "1"); // apply rounded corners to UTF8-styled tables.
        env::set_var("POLARS_FMT_MAX_COLS", "20"); // maximum number of columns shown when formatting DataFrames.
        env::set_var("POLARS_FMT_MAX_ROWS", "20"); // maximum number of rows shown when formatting DataFrames.
        env::set_var("POLARS_FMT_STR_LEN", "50"); // maximum number of characters printed per string value.
    }

    #[derive(Debug, Parser)]
    #[command(author, version, about, long_about = None, arg_required_else_help = true)]
    struct Cli {
        /// The path to the alternatives file
        #[arg(short, long)]
        alternatives: PathBuf,

        /// The path to the criteria file
        #[arg(short, long)]
        criteria: PathBuf,
    }

    let args: Cli = Cli::parse();

    let mut data_df = interop::polars::df_from_csv(
        args.alternatives
            .to_str()
            .expect("failed to convert data path to str"),
    )
    .expect("failed to load data");

    let criteria_df: DataFrame = interop::polars::df_from_csv(
        args.criteria
            .to_str()
            .expect("failed to convert criteria path to str"),
    )
    .expect("failed to load criteria");

    let mut p: Prom = interop::polars::prom_from_polars(&data_df, &criteria_df).unwrap();
    let now: Instant = Instant::now();
    p.compute_prom_ii().expect("failed to compute prom.");
    let timing = now.elapsed().as_secs_f64();

    let score = Series::new("score", p.prom_ii.as_ref().unwrap().score.clone());
    let normalized_score = Series::new(
        "normalized_score",
        p.prom_ii.as_ref().unwrap().normalized_score.clone(),
    );
    data_df.with_column(score).unwrap();
    data_df.with_column(normalized_score).unwrap();

    configure_the_environment();
    println!("Calculation time (ms):  {:.2}", timing * 1000.0);
    println!("Scoring Criteria {:#?}", criteria_df);
    println!(
        "Data with Prom II Scores {:#?}",
        data_df.sort(["score"], true, false).unwrap()
    );
}

fn main() {
    if cfg!(feature = "io") && cfg!(feature = "cli") {
        #[cfg(all(feature = "io", feature = "cli"))]
        run_cli();
    } else {
        println!("`cli` and `io` feature are required for command line use.");
    }
}
