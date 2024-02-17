/// This module make it possible to load your data from a polars dataframe.
use super::types::{Arr, Criteria, Mat};
use super::Prom;
#[cfg(feature = "io")]
pub mod polars {
    use super::*;
    extern crate polars;
    use polars::prelude::*;
    use std::fmt::Error;

    pub fn df_from_csv(filename: &str) -> Result<DataFrame, PolarsError> {
        CsvReader::from_path(filename)?.has_header(true).finish()
    }

    pub fn series_to_vec_f32(ser: &Series) -> Result<Arr, PolarsError> {
        let new = ser
            .cast(&DataType::Float32)?
            .f32()?
            .into_no_null_iter()
            .collect();
        Ok(new)
    }

    pub fn series_to_vec_string(ser: &Series) -> Result<Vec<String>, PolarsError> {
        let new = ser
            .str()?
            .into_iter()
            .map(|s| match s {
                Some(s) => s.to_string(),
                None => "usual".to_string(),
            })
            .collect();
        Ok(new)
    }

    pub fn df_to_matrix(df: &DataFrame) -> Result<Mat, PolarsError> {
        let (_, m) = df.shape();
        let mut matrix: Mat = Vec::new();

        for i in 1..m {
            matrix.push(series_to_vec_f32(&df[i]).unwrap());
        }

        Ok(matrix)
    }

    pub fn df_to_criteria(df: &DataFrame) -> Result<Criteria, PolarsError> {
        Ok(Criteria {
            weight: series_to_vec_f32(&df["weight"]).expect("a weight column is required"),
            criteria_type: series_to_vec_f32(&df["criteria_type"])
                .expect("a criteria_type column is required"),
            pref_function: series_to_vec_string(&df["pref_function"])
                .expect("a pref_function column is required"),
            q: series_to_vec_f32(&df["q"]).expect("a q column is required"),
            p: series_to_vec_f32(&df["p"]).expect("a q column is required"),
        })
    }

    pub fn prom_from_polars(data_df: &DataFrame, criteria_df: &DataFrame) -> Result<Prom, Error> {
        let matrix_t = df_to_matrix(data_df).expect("unable to convert matrix.");
        let criteria = df_to_criteria(criteria_df).expect("unable to load criteria");

        Ok(Prom {
            matrix_t,
            criteria,
            mc_flow: None,
            prom_i: None,
            prom_ii: None,
        })
    }
}
