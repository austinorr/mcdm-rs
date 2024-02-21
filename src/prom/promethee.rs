use super::math::{diff, min_max_norm, mult_axis_0, normalize_vec, sum_axis_0, transpose};
use super::multicriterion_flow::multicriterion_flow;
use super::types::{Arr, Criteria, Fl, MCFlowResult, Mat, PromResultI, PromResultII};
use std::fmt::Error;

pub fn prom_i(pref_matrix_plus_t: &Mat, pref_matrix_minus_t: &Mat, weights: &Arr) -> PromResultI {
    let phi_plus_matrix: Mat = transpose(mult_axis_0(pref_matrix_plus_t, weights));
    let phi_minus_matrix: Mat = transpose(mult_axis_0(pref_matrix_minus_t, weights));

    PromResultI {
        phi_plus_score: sum_axis_0(&phi_plus_matrix),
        phi_minus_score: sum_axis_0(&phi_minus_matrix),
        phi_plus_matrix,
        phi_minus_matrix,
    }
}

pub fn prom_ii(p: &PromResultI) -> PromResultII {
    // TODO: use ndarray for a vectorized diff
    let score: Arr = diff(&p.phi_plus_score, &p.phi_minus_score);
    let normalized_score = min_max_norm(&score);

    // TODO: use ndarray for a matrix diff
    let weighted_flow: Mat = p
        .phi_plus_matrix
        .iter()
        .zip(&p.phi_minus_matrix)
        .map(|(a, b)| diff(a, b))
        .collect();

    PromResultII {
        score,
        normalized_score,
        weighted_flow,
    }
}

#[derive(Clone, Default, Debug)]
pub struct Prom {
    pub matrix_t: Mat,
    pub criteria: Criteria,
    pub mc_flow: Option<MCFlowResult>,
    pub prom_i: Option<PromResultI>,
    pub prom_ii: Option<PromResultII>,
}

impl Prom {
    /// Returns a new Promethee analysis struct.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use mcdmrs::prom::Prom;
    /// let mut p: Prom = Prom::new(
    ///     vec![vec![0.8, 0.2, 0.05], vec![0.1, 0.6, 0.4]],
    ///     vec![1., 1.],
    ///     vec![-1., 1.],
    ///     vec!["usual".to_string(), "usual".to_string()],
    ///     vec![0., 0.],
    ///     vec![0., 0.],
    /// );
    /// ```
    pub fn new(
        matrix_t: Mat,
        weight: Arr,
        criteria_type: Arr,
        pref_function: Vec<String>,
        q: Arr,
        p: Arr,
    ) -> Self {
        Prom {
            matrix_t,
            criteria: Criteria {
                weight,
                criteria_type,
                pref_function,
                q,
                p,
            },
            mc_flow: None,
            prom_i: None,
            prom_ii: None,
        }
    }

    pub fn compute_multicriterion_flow(&mut self) -> Result<(), Error> {
        let (pref_matrix_plus_t, pref_matrix_minus_t) = multicriterion_flow(
            &mult_axis_0(&self.matrix_t, &self.criteria.criteria_type),
            &self.criteria.pref_function,
            &self.criteria.q,
            &self.criteria.p,
        );

        self.mc_flow = Some(MCFlowResult {
            pref_matrix_plus_t,
            pref_matrix_minus_t,
        });

        Ok(())
    }

    pub fn compute_prom_i(&mut self) -> Result<(), Error> {
        match self.mc_flow {
            None => {
                _ = self.compute_multicriterion_flow();
                _ = self.compute_prom_i();
            }
            _ => {
                self.prom_i = Some(prom_i(
                    &self.mc_flow.as_ref().unwrap().pref_matrix_plus_t,
                    &self.mc_flow.as_ref().unwrap().pref_matrix_minus_t,
                    &normalize_vec(&self.criteria.weight),
                ));
            }
        }

        Ok(())
    }

    pub fn compute_prom_ii(&mut self) -> Result<(), Error> {
        match self.prom_i {
            None => {
                _ = self.compute_prom_i();
                _ = self.compute_prom_ii();
            }
            _ => {
                self.prom_ii = Some(prom_ii(self.prom_i.as_ref().unwrap()));
            }
        }

        Ok(())
    }

    pub fn re_weight(&mut self, weight: &[Fl]) -> Result<(), Error> {
        self.criteria.weight = weight.to_vec();
        self.prom_i = None;
        self.compute_prom_ii()
    }
}

#[allow(clippy::type_complexity)]
#[allow(clippy::excessive_precision)]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_prom() {
        use is_close::all_close;

        let _p = Prom::default();

        let _p: Prom = Prom {
            matrix_t: vec![vec![0.8, 0.2, 0.5], vec![0.8, 0.2, 0.5]],
            criteria: Criteria {
                weight: vec![1., 1.],
                criteria_type: vec![-1., 1.],
                pref_function: vec!["usual".to_string(), "usual".to_string()],
                q: vec![0., 0.],
                p: vec![0., 0.],
            },
            mc_flow: None,
            prom_i: None,
            prom_ii: None,
        };

        let mut p: Prom = Prom::new(
            vec![vec![0.8, 0.2, 0.05], vec![0.1, 0.6, 0.4]],
            vec![1., 1.],
            vec![-1., 1.],
            vec!["usual".to_string(), "usual".to_string()],
            vec![0., 0.],
            vec![0., 0.],
        );
        println!("test prom new: {:#?}", p);

        _ = p.compute_multicriterion_flow();
        _ = p.compute_prom_i();
        _ = p.compute_prom_ii();
        println!("test prom calc: {:#?}", p);

        let score: Arr = p.prom_ii.clone().unwrap().score;
        assert!(all_close!(
            vec![-1., 0.5, 0.5],
            score.clone(),
            rel_tol = 1e-6
        ));

        _ = p.re_weight(&[0.75, 0.25]);
        println!("test prom re-weight: {:#?}", p);

        let newscore: Arr = p.prom_ii.clone().unwrap().score;
        assert!(!all_close!(newscore.clone(), score.clone(), rel_tol = 1e-6));
    }

    fn get_prom_inputs() -> (Mat, Arr, Arr, Vec<String>, Arr, Arr) {
        let mat = vec![
            vec![-2.51, 9.01, 4.64, 1.97, -6.88, -6.88, -8.84, 7.32],
            vec![2.02, 4.16, -9.59, 9.4, 6.65, -5.75, -6.36, -6.33],
            vec![-3.92, 0.5, -1.36, -4.18, 2.24, -7.21, -4.16, -2.67],
            vec![-0.88, 5.7, -6.01, 0.28, 1.85, -9.07, 2.15, -6.59],
            vec![-8.7, 8.98, 9.31, 6.17, -3.91, -8.05, 3.68, -1.2],
            vec![-7.56, -0.1, -9.31, 8.19, -4.82, 3.25, -3.77, 0.4],
            vec![0.93, -6.3, 9.39, 5.5, 8.79, 7.9, 1.96, 8.44],
            vec![-8.23, -6.08, -9.1, -3.49, -2.23, -4.57, 6.57, -2.86],
            vec![-4.38, 0.85, -7.18, 6.04, -8.51, 9.74, 5.44, -6.03],
            vec![-9.89, 6.31, 4.14, 4.58, 5.43, -8.52, -2.83, -7.68],
            vec![7.26, 2.47, -3.38, -8.73, -3.78, -3.5, 4.59, 2.75],
            vec![7.74, -0.56, -7.61, 4.26, 5.22, 1.23, 5.42, -0.12],
            vec![0.45, -1.45, -9.49, -7.84, -9.37, 2.73, -3.71, 0.17],
            vec![8.15, -5.01, -1.79, 5.11, -5.42, -8.46, -4.2, -6.78],
            vec![8.59, 6.16, 2.67, 7.43, 6.07, -6.27, 7.85, 0.79],
            vec![6.15, 7.92, -3.64, -7.8, -5.44, -1.46, 6.36, 7.21],
            vec![-9.86, 0.21, -1.65, -5.56, -7.6, -3.25, 8.86, -3.54],
            vec![0.38, 4.06, -2.73, 9.44, 9.25, -4.96, -0.06, -3.98],
            vec![-4.3, -9.26, 2.19, 0.05, -8.97, -4.43, 8.17, -5.21],
            vec![-7.1, -0.21, 9.71, -5.16, 3.44, 5.23, -5.25, 4.56],
        ];

        let weights: Arr = vec![0.11, 0.157, 0.158, 0.14, 0.061, 0.194, 0.102, 0.078];

        let criteria_types: Arr = vec![-1., -1., 1., 1., -1., 1., -1., 1.];
        let prefs: Vec<String> = [
            "vshape2", "usual", "ushape", "vshape", "usual", "level", "vshape2", "usual",
        ]
        .map(String::from)
        .to_vec();

        let q: Arr = vec![0.37, 0.95, 0.73, 0.6, 0.16, 0.16, 0.06, 0.87];
        let p: Arr = vec![0.6, 0.71, 0.02, 0.97, 0.83, 0.21, 0.18, 0.18];

        {
            (mat, weights, criteria_types, prefs, q, p)
        }
    }

    #[test]
    fn test_complex_prom_usual() {
        use is_close::all_close;
        let (matrix, weights, criteria_types, _prefs, q, p) = get_prom_inputs();

        let prefs = vec!["usual".to_string(); weights.len()];

        let exp_promii: Arr = vec![
            0.06473685,
            -0.20399999,
            -0.07252632,
            -0.43105263,
            0.01705263,
            0.31663158,
            0.49526317,
            -0.05600001,
            0.17242105,
            -0.14389474,
            -0.2128421,
            -0.05884212,
            0.02673685,
            -0.07652631,
            -0.18705264,
            -0.2368421,
            0.02263158,
            -0.03126316,
            0.14389474,
            0.45147368,
        ];

        let mut p = Prom::new(transpose(matrix), weights, criteria_types, prefs, q, p);

        let _ = p.compute_prom_ii();
        let score = p.prom_ii.clone().unwrap().score;
        println!("expected: {:#?} got: {:#?}", exp_promii, score);
        assert!(all_close!(exp_promii, score, abs_tol = 1e-3))
    }

    #[test]
    fn test_complex_prom_ushape() {
        use is_close::all_close;
        let (matrix, weights, criteria_types, _prefs, q, p) = get_prom_inputs();

        let prefs = vec!["ushape".to_string(); weights.len()];

        let exp_promii: Arr = vec![
            0.06057895,
            -0.1675263,
            -0.07547368,
            -0.45084211,
            0.0148421,
            0.29057895,
            0.48699999,
            -0.06247369,
            0.18026316,
            -0.11463157,
            -0.20963158,
            -0.0568421,
            0.04942106,
            -0.05994736,
            -0.20357895,
            -0.235,
            0.04815789,
            -0.05689473,
            0.15105263,
            0.41094736,
        ];

        let mut p = Prom::new(transpose(matrix), weights, criteria_types, prefs, q, p);

        let _ = p.compute_prom_ii();
        let score = p.prom_ii.clone().unwrap().score;
        println!("expected: {:#?} got: {:#?}", exp_promii, score);
        assert!(all_close!(exp_promii, score, abs_tol = 1e-3))
    }

    #[test]
    fn test_complex_prom_vshape() {
        use is_close::all_close;
        let (matrix, weights, criteria_types, _prefs, q, p) = get_prom_inputs();

        let prefs = vec!["vshape".to_string(); weights.len()];

        let exp_promii: Arr = vec![
            0.07147984,
            -0.18886907,
            -0.05906976,
            -0.44244925,
            -0.00348323,
            0.31576549,
            0.49052921,
            -0.04776636,
            0.18007082,
            -0.13397242,
            -0.21038076,
            -0.06793786,
            0.03906552,
            -0.08500622,
            -0.1877535,
            -0.2465595,
            0.03436363,
            -0.04959235,
            0.14923934,
            0.44232639,
        ];

        let mut p = Prom::new(transpose(matrix), weights, criteria_types, prefs, q, p);

        let _ = p.compute_prom_ii();
        let score = p.prom_ii.clone().unwrap().score;
        println!("expected: {:#?} got: {:#?}", exp_promii, score);
        assert!(all_close!(exp_promii, score, abs_tol = 1e-3))
    }

    #[test]
    fn test_complex_prom_vshape2() {
        use is_close::all_close;
        let (matrix, weights, criteria_types, _prefs, q, p) = get_prom_inputs();

        let prefs = vec!["vshape2".to_string(); weights.len()];

        let exp_promii: Arr = vec![
            0.06942184,
            -0.18717045,
            -0.03388529,
            -0.45705578,
            -0.01103617,
            0.30566547,
            0.49448485,
            -0.03983012,
            0.18597868,
            -0.11989114,
            -0.2102624,
            -0.07571162,
            0.03858889,
            -0.09083485,
            -0.17913062,
            -0.24511867,
            0.03526237,
            -0.05216981,
            0.14159187,
            0.43110296,
        ];

        let mut p = Prom::new(transpose(matrix), weights, criteria_types, prefs, q, p);

        let _ = p.compute_prom_ii();
        let score = p.prom_ii.clone().unwrap().score;
        println!("expected: {:#?} got: {:#?}", exp_promii, score);
        assert!(all_close!(exp_promii, score, abs_tol = 1e-3))
    }

    #[test]
    fn test_complex_prom_level() {
        use is_close::all_close;
        let (matrix, weights, criteria_types, _prefs, q, p) = get_prom_inputs();

        let prefs = vec!["level".to_string(); weights.len()];

        let exp_promii: Arr = vec![
            0.0705,
            -0.18676315,
            -0.03955264,
            -0.45655263,
            -0.00836842,
            0.30789475,
            0.49118422,
            -0.0383158,
            0.18355263,
            -0.11926315,
            -0.20515789,
            -0.07128949,
            0.03707895,
            -0.0905,
            -0.18047369,
            -0.24860526,
            0.03418421,
            -0.05397368,
            0.14394737,
            0.43047367,
        ];

        let mut p = Prom::new(transpose(matrix), weights, criteria_types, prefs, q, p);

        let _ = p.compute_prom_ii();
        let score = p.prom_ii.clone().unwrap().score;
        println!("expected: {:#?} got: {:#?}", exp_promii, score);
        assert!(all_close!(exp_promii, score, abs_tol = 1e-3))
    }

    #[test]
    fn test_complex_prom_all() {
        use is_close::all_close;
        let (matrix, weights, criteria_types, prefs, q, p) = get_prom_inputs();

        let exp_promii: Arr = vec![
            0.05642106,
            -0.17198806,
            -0.07260072,
            -0.4366739, // <- min
            0.02175211,
            0.29782811,
            0.50030197, // <- max
            -0.06959151,
            0.1629708,
            -0.13237331,
            -0.21495115,
            -0.05180758,
            0.0496468,
            -0.07426273,
            -0.18974575,
            -0.23589474,
            0.03275095,
            -0.05369158,
            0.14787299,
            0.43403623,
        ];

        let mut p = Prom::new(transpose(matrix), weights, criteria_types, prefs, q, p);

        let _ = p.compute_prom_ii();
        let score = p.prom_ii.clone().unwrap().score;
        println!("expected: {:#?} got: {:#?}", exp_promii, score);
        assert!(all_close!(
            exp_promii,
            score,
            rel_tol = 1e-6,
            abs_tol = 1e-3
        ))
    }
}
