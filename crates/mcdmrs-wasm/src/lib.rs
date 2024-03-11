// use js_sys;
use mcdmrs_prom::{types::Fl, Criteria, Prom};
use ndarray::{Array1, Array2};
use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsError;
pub use wasm_bindgen_rayon::init_thread_pool;

// #[derive(Debug, Clone)]
// struct MCDMRSError {
//     details: String,
// }

// impl MCDMRSError {
//     fn new(msg: &str) -> MCDMRSError {
//         MCDMRSError {
//             details: msg.to_string(),
//         }
//     }
// }

// impl fmt::Display for MCDMRSError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.details)
//     }
// }

// impl Error for MCDMRSError {
//     fn description(&self) -> &str {
//         &self.details
//     }
// }

// pub type AnyError = Box<dyn std::error::Error>;

// impl From<AnyError> for JsError {
//     fn from(e: Box<dyn std::error::Error>) -> Self {
//         JsError::new(&format!("{}", e))
//     }
// }

// type Result<T> = std::result::Result<T, JsError>;

// impl From<Box<dyn std::error::Error>> for JsError {
//     fn from(e: Box<dyn std::error::Error>) -> Self {
//         JsError::new(&format!("{}", e))
//     }
// }

// impl<E> Into<JsValue> for E
// where
//     E: std::error::Error,
// {
//     #[inline]
//     fn into(value: E) -> js_sys::Error {
//         js_sys::Error::new(&value.to_string())
//     }
// }

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct PromJS {
    _prom: Prom,
}

#[wasm_bindgen]
impl PromJS {
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(constructor)]
    pub fn new(
        matrix_t_val: Vec<f32>,
        ncol: usize, // alternatives
        nrow: usize, // criteria
        weight: Vec<f32>,
        criteria_type: Vec<f32>,
        pref_function: Vec<String>,
        q: Vec<f32>,
        p: Vec<f32>,
    ) -> Result<PromJS, JsError> {
        let matrix_t1d = matrix_t_val.clone();

        let mat_t: Array2<Fl> = Array1::from_vec(matrix_t1d).into_shape((nrow, ncol))?;

        let criteria = Criteria::new(
            Array1::from_vec(weight),
            Array1::from_vec(criteria_type),
            Array1::<String>::from_vec(pref_function),
            Array1::from_vec(q),
            Array1::from_vec(p),
        )
        .map_err(|err| JsError::new(&err.to_string()))?;
        // .expect("can't create criteria");

        let prom = Prom::new(mat_t, criteria).map_err(|err| JsError::new(&err.to_string()))?;

        Ok(PromJS { _prom: prom })
    }

    pub fn compute_prom_ii(&mut self) {
        _ = self._prom.compute_prom_ii();
    }

    pub fn get_score(&mut self) -> Vec<Fl> {
        match &self._prom.prom_ii {
            Some(result) => result.score.to_vec(),
            _ => vec![0.0; self._prom.matrix_t.dim().1],
        }
    }

    pub fn re_weight(&mut self, weight: Vec<f32>) {
        _ = self._prom.re_weight(Array1::from_vec(weight).view());
    }
}

// #[allow(clippy::too_many_arguments)]
// #[wasm_bindgen]
// pub fn promethee(
//     matrix_t_val: Vec<f32>,
//     ncol: usize, // alternatives
//     nrow: usize, // criteria
//     weight: Vec<f32>,
//     criteria_type: Vec<f32>,
//     pref_function: Vec<String>,
//     q: Vec<f32>,
//     p: Vec<f32>,
// ) {
//     let matrix_t1d = matrix_t_val.clone(); //: Matrix2DF32 = serde_wasm_bindgen::from_value(matrix_t_val).unwrap();

//     log(&format!("{:#?}", &matrix_t1d));

//     let mat_t: Array2<Fl> = Array1::from_vec(matrix_t1d)
//         .into_shape((nrow, ncol))
//         .unwrap();

//     log(&format!("{:#?}", mat_t));

//     let mut p = Prom {
//         matrix_t: mat_t,
//         criteria: Criteria {
//             weight: Array1::from_vec(weight),
//             criteria_type: Array1::from_vec(criteria_type),
//             pref_function: Array1::<String>::from_vec(pref_function),
//             q: Array1::from_vec(q),
//             p: Array1::from_vec(p),
//         },
//         ..Default::default()
//     };

//     // p

//     _ = p.compute_prom_ii();

//     log(&format!("prom {:#?}", p));

//     // alert(&format!("prom: {:#?}!", p));
// }
