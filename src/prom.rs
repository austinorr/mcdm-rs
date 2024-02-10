use std::fmt::Error;

use crate::matrix::{diff, normalize_vec, sum_axis_0, transpose};
use crate::types::*;

pub fn apply_weights(pref_matrix_t: &[Arr], weights: &[Fl]) -> Mat {
    // TODO: replace with ndarray mult
    let mut out: Mat = vec![vec![0.; pref_matrix_t[0].len()]; pref_matrix_t.len()];
    for (i, col) in pref_matrix_t.iter().enumerate() {
        out[i] = col.to_vec();
        if weights[i] != 0. {
            out[i].iter_mut().for_each(|x| *x *= weights[i]);
        }
    }
    out
}

#[test]
fn test_apply_weights() {
    let mat: Mat = vec![vec![0., 1., 0.5], vec![1., 0., 0.5]];
    let weights: Arr = vec![1., 2.];

    let new_mat: Mat = apply_weights(&mat, &weights);
    let exp: Mat = vec![vec![0.0, 1.0, 0.5], vec![2.0, 0.0, 1.0]];
    assert_eq!(exp, new_mat);
}

#[derive(Clone, Debug)]
pub struct MCFlowResult {
    pub pref_matrix_plus_t: Mat,
    pub pref_matrix_minus_t: Mat,
}

#[derive(Clone, Debug)]
pub struct PromResultI {
    pub phi_plus_score: Arr,
    pub phi_minus_score: Arr,
    pub phi_plus_matrix: Mat,
    pub phi_minus_matrix: Mat,
}

pub fn prom_i(pref_matrix_plus_t: &Mat, pref_matrix_minus_t: &Mat, weights: &[Fl]) -> PromResultI {
    let phi_plus_matrix: Mat = transpose(apply_weights(&pref_matrix_plus_t, &weights));
    let phi_minus_matrix: Mat = transpose(apply_weights(&pref_matrix_minus_t, &weights));

    PromResultI {
        phi_plus_score: sum_axis_0(&phi_plus_matrix),
        phi_minus_score: sum_axis_0(&phi_minus_matrix),
        phi_plus_matrix,
        phi_minus_matrix,
    }
}

#[derive(Clone, Debug)]
pub struct PromResultII {
    pub score: Arr,
    pub weighted_flow: Mat,
}

pub fn prom_ii(p: &PromResultI) -> PromResultII {
    // TODO: use ndarray for a vectorized diff
    let score: Arr = diff(&p.phi_plus_score, &p.phi_minus_score);

    // TODO: use ndarray for a matrix diff
    let weighted_flow: Mat = p
        .phi_plus_matrix
        .iter()
        .zip(&p.phi_minus_matrix)
        .map(|(a, b)| diff(&a, &b))
        .collect();

    PromResultII {
        score,
        weighted_flow,
    }
}

#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct Prom {
    pub matrix_t: Mat,
    weights: Arr,
    criteria_type: Arr,
    pref_functions: Vec<String>,
    q: Arr,
    p: Arr,
    mc_flow: Option<MCFlowResult>,
    prom_i: Option<PromResultI>,
    prom_ii: Option<PromResultII>,
}

impl Prom {
    pub fn new(
        matrix_t: &Mat,
        weights: Arr,
        criteria_type: Arr,
        pref_functions: Vec<String>,
        q: Arr,
        p: Arr,
    ) -> Result<Self, Error> {
        let mut mat = matrix_t.clone();

        for (i, col) in mat.iter_mut().enumerate() {
            for x in col.iter_mut() {
                *x *= criteria_type[i];
            }
        }

        Ok(Prom {
            matrix_t: mat,
            weights: normalize_vec(&weights),
            criteria_type,
            pref_functions,
            q,
            p,
            mc_flow: None,
            prom_i: None,
            prom_ii: None,
        })
    }

    pub fn compute_multicriterion_flow(&mut self) -> Result<(), Error> {
        use crate::multicriterion_flow::multicriterion_flow;
        let (pref_matrix_plus_t, pref_matrix_minus_t) =
            multicriterion_flow(&self.matrix_t, &self.pref_functions, &self.q, &self.p);

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
                    &self.weights,
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
                self.prom_ii = Some(prom_ii(&self.prom_i.as_ref().unwrap()));
            }
        }

        Ok(())
    }

    pub fn re_weight(&mut self, weights: &[Fl]) -> Result<(), Error> {
        self.weights = normalize_vec(&weights);
        self.prom_i = None;
        self.compute_prom_ii()
    }
}

#[test]
fn test_prom() {
    use is_close::all_close;

    let _p = Prom::default();
    // println!("test prom default: {:#?}", p);

    let _p: Prom = Prom {
        matrix_t: vec![vec![0.8, 0.2, 0.5], vec![0.8, 0.2, 0.5]],
        weights: vec![1., 1.],
        criteria_type: vec![-1., 1.],
        pref_functions: vec!["usual".to_string(), "usual".to_string()],
        q: vec![0., 0.],
        p: vec![0., 0.],
        mc_flow: None,
        prom_i: None,
        prom_ii: None,
    };

    let mut p: Prom = Prom::new(
        &vec![vec![0.8, 0.2, 0.05], vec![0.1, 0.6, 0.4]],
        vec![1., 1.],
        vec![-1., 1.],
        vec!["usual".to_string(), "usual".to_string()],
        vec![0., 0.],
        vec![0., 0.],
    )
    .expect("unable to build with Prom::new");
    println!("test prom new: {:#?}", p);

    _ = p.compute_multicriterion_flow();
    _ = p.compute_prom_i();
    _ = p.compute_prom_ii();
    println!("test prom calc: {:#?}", p);

    assert!(all_close!(
        vec![-1., 0.5, 0.5],
        p.prom_ii.expect("error getting prom_ii").score,
        rel_tol = 1e-6
    ))

    // _ = p.re_weight(&vec![0.75, 0.25]);
    // println!("test prom re-weight: {:#?}", p);
}
