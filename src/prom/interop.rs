/// This module make it possible to load your data from a polars dataframe.
use super::{Criteria, Prom};
use ndarray::{Array1, Axis};

#[cfg(feature = "io")]
pub mod polars {
    use super::*;
    use crate::prom::{Criteria, Prom};
    extern crate polars;
    use polars::prelude::{
        CsvReader, DataFrame, Float32Type, IndexOrder, PolarsError, PolarsResult, SerReader, Series,
    };

    pub fn df_from_csv(filename: &str) -> PolarsResult<DataFrame> {
        CsvReader::from_path(filename)?.has_header(true).finish()
    }

    fn _pref_func_to_vec_string(ser: &Series) -> Result<Vec<String>, PolarsError> {
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

    fn _series_to_vec_string(ser: &Series) -> Result<Vec<String>, PolarsError> {
        let new = ser
            .str()?
            .into_iter()
            .map(|s| s.unwrap().to_string())
            .collect();
        Ok(new)
    }

    pub fn df_to_criteria(df: &DataFrame) -> Result<Criteria, PolarsError> {
        let float_df = df.select(["weight", "criteria_type", "q", "p"])?;
        let float_array = float_df.to_ndarray::<Float32Type>(IndexOrder::C)?;

        Ok(Criteria {
            weight: float_array
                .index_axis(Axis(1), float_df.get_column_index("weight").unwrap())
                .to_owned(),
            criteria_type: float_array
                .index_axis(Axis(1), float_df.get_column_index("criteria_type").unwrap())
                .to_owned(),
            pref_function: Array1::<String>::from_vec(
                _pref_func_to_vec_string(df.column("pref_function")?).unwrap(),
            ),
            q: float_array
                .index_axis(Axis(1), float_df.get_column_index("q").unwrap())
                .to_owned(),
            p: float_array
                .index_axis(Axis(1), float_df.get_column_index("p").unwrap())
                .to_owned(),
        })
    }

    pub fn prom_from_polars(
        data_df: &DataFrame,
        criteria_df: &DataFrame,
    ) -> Result<Prom, PolarsError> {
        let matrix_t = data_df
            .select(_series_to_vec_string(criteria_df.column("name")?).unwrap())?
            .to_ndarray::<Float32Type>(IndexOrder::C)
            .expect("unable to convert matrix.")
            .t()
            .to_owned();
        let criteria = df_to_criteria(criteria_df).expect("unable to load criteria");

        Ok(Prom {
            matrix_t,
            criteria,
            mc_flow: None,
            prom_i: None,
            prom_ii: None,
        })
    }

    pub trait FromPolars {
        fn from_polars(data_df: &DataFrame, criteria_df: &DataFrame) -> Result<Prom, PolarsError>;
    }

    impl FromPolars for Prom {
        fn from_polars(data_df: &DataFrame, criteria_df: &DataFrame) -> Result<Prom, PolarsError> {
            prom_from_polars(data_df, criteria_df)
        }
    }
}
