use numpy::ndarray::Array1;
use numpy::{IntoPyArray, PyArray2, PyReadonlyArray1, PyReadonlyArray2};
use pyo3::{pymodule, types::PyModule, PyResult, Python};

use mcdmrs_prom::multicriterion_flow;
use mcdmrs_prom::types::Fl;

#[pymodule]
fn _mcdmrs<'py>(_py: Python<'py>, m: &'py PyModule) -> PyResult<()> {
    // wrapper of `multicriterion_flow`
    #[pyfn(m)]
    #[pyo3(name = "_multicriterion_flow")]
    fn multicriterion_flow_py<'py>(
        py: Python<'py>,
        matrix_t: PyReadonlyArray2<'py, Fl>,
        pref_function: Vec<String>,
        q: PyReadonlyArray1<'py, Fl>,
        p: PyReadonlyArray1<'py, Fl>,
    ) -> PyResult<(&'py PyArray2<Fl>, &'py PyArray2<Fl>)> {
        let prefs: Array1<String> = pref_function.into();

        let res = multicriterion_flow(
            matrix_t.as_array(),
            prefs.view(),
            q.as_array(),
            p.as_array(),
        )
        .expect("unable to compute");

        let plus = res.pref_matrix_plus_t.into_pyarray(py);
        let minus = res.pref_matrix_minus_t.into_pyarray(py);

        Ok((plus, minus))
    }

    Ok(())
}
