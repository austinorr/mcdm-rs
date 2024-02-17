use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    /// The path to the alternatives file
    #[arg(short, long)]
    alternatives: std::path::PathBuf,

    /// The path to the criteria file
    #[arg(short, long)]
    criteria: std::path::PathBuf,
}

fn main() {
    fn configure_the_environment() {
        use std::env;
        env::set_var("POLARS_FMT_TABLE_ROUNDED_CORNERS", "1"); // apply rounded corners to UTF8-styled tables.
        env::set_var("POLARS_FMT_MAX_COLS", "20"); // maximum number of columns shown when formatting DataFrames.
        env::set_var("POLARS_FMT_MAX_ROWS", "20"); // maximum number of rows shown when formatting DataFrames.
        env::set_var("POLARS_FMT_STR_LEN", "50"); // maximum number of characters printed per string value.
    }

    if cfg!(feature = "io") && cfg!(feature = "cli") {
        use mcdmrs::prom::{interop, Prom};
        use polars::prelude::*;
        use std::time::Instant;

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
    } else {
        println!("`cli` and `io` feature are required for command line use.");
    }
}
