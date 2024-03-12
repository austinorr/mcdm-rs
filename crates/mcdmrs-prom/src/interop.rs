/// This module make it possible to load your data from a polars dataframe.
#[cfg(feature = "io")]
pub mod polars {
    use super::super::{Criteria, Prom, Result};
    use ndarray::{Array1, Axis};
    use polars::prelude::{
        CsvReader, DataFrame, Float32Type, IndexOrder, PolarsResult, SerReader, Series,
    };

    pub fn df_from_csv(filename: &str) -> PolarsResult<DataFrame> {
        CsvReader::from_path(filename)?.has_header(true).finish()
    }

    fn _pref_func_to_vec_string(ser: &Series) -> Result<Vec<String>> {
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

    fn _series_to_vec_string(ser: &Series) -> Result<Vec<String>> {
        let new = ser
            .str()?
            .into_iter()
            .map(|s| s.unwrap().to_string())
            .collect();
        Ok(new)
    }

    pub fn df_to_criteria(df: &DataFrame) -> Result<Criteria> {
        let float_df = df.select(["weight", "criteria_type", "q", "p"])?;
        let float_array = float_df.to_ndarray::<Float32Type>(IndexOrder::C)?;

        Ok(Criteria {
            weight: float_array
                .index_axis(Axis(1), float_df.get_column_index("weight").unwrap())
                .to_owned(),
            criteria_type: float_array
                .index_axis(Axis(1), float_df.get_column_index("criteria_type").unwrap())
                .to_owned(),
            pref_function: Array1::<String>::from_vec(_pref_func_to_vec_string(
                df.column("pref_function")?,
            )?),
            q: float_array
                .index_axis(Axis(1), float_df.get_column_index("q").unwrap())
                .to_owned(),
            p: float_array
                .index_axis(Axis(1), float_df.get_column_index("p").unwrap())
                .to_owned(),
        })
    }

    pub fn prom_from_polars(data_df: &DataFrame, criteria_df: &DataFrame) -> Result<Prom> {
        let matrix_t = data_df
            .select(_series_to_vec_string(criteria_df.column("name")?)?)?
            .to_ndarray::<Float32Type>(IndexOrder::C)?
            .t()
            .to_owned();
        let criteria = df_to_criteria(criteria_df)?;

        Ok(Prom {
            matrix_t,
            criteria,
            mc_flow: None,
            prom_i: None,
            prom_ii: None,
        })
    }

    pub trait FromPolars {
        fn from_polars(data_df: &DataFrame, criteria_df: &DataFrame) -> Result<Prom>;
    }

    impl FromPolars for Prom {
        fn from_polars(data_df: &DataFrame, criteria_df: &DataFrame) -> Result<Prom> {
            prom_from_polars(data_df, criteria_df)
        }
    }

    #[cfg(test)]
    mod test {

        use super::*;
        use polars::prelude::*;

        #[test]
        fn test_from_polars() -> Result<()> {
            let criteria_df: DataFrame = df!(
                "name"=> &["one", "two"],
                "weight" => &[1., 1.],
                "criteria_type" => &[-1., 1.],
                "pref_function" => &["usual", "ushape"],
                "q" => &[0., 0.],
                "p" => &[0., 0.],
            )?;

            let data_df: DataFrame = df!(
                "one"=> &[0.8, 0.2, 0.05],
                "two" => &[0.1, 0.6, 0.4],

            )?;

            let mut p = Prom::from_polars(&data_df, &criteria_df)?;
            p.compute_prom_ii()?;

            println!("{:#?}", p.prom_ii);

            Ok(())
        }

        #[test]
        fn test_from_polars_missing_col() -> Result<()> {
            let criteria_df: DataFrame = df!(
                "name"=> &["one", "two"],
                "weight" => &[1., 1.],
                "criteria_type" => &[-1., 1.],
                "pref_function" => &["usual", "ushape"],
                "q" => &[0., 0.],
                "p" => &[0., 0.],
            )?;

            let data_df: DataFrame = df!(
                "one"=> &[0.8, 0.2, 0.05],
                "two" => &[0.1, 0.6, 0.4],
                "three" => &[0.2, 0.5, 0.4],

            )?;

            let mut p = Prom::from_polars(&data_df, &criteria_df)?;
            p.compute_prom_ii()?;

            println!("{:#?}", p.prom_ii);

            Ok(())
        }
    }
}
