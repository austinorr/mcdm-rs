use crate::multicriterion_flow::multicriterion_flow;
use crate::types::*;

pub fn norm_vec(vec: &[Fl]) -> Arr {
    let s: Fl = vec.iter().sum();
    let mut b: Arr = vec.to_vec();
    if s != 0. {
        b.iter_mut().for_each(|x| *x /= s);
    }
    b
}

#[test]
fn test_norm_vec() {
    let vec: Vec<f32> = vec![1., 0.0, 2.0];
    assert_eq!(vec![0.33333334, 0.0, 0.6666667], norm_vec(&vec));

    let vec: Vec<f32> = vec![0., 0.0, 0.0];
    assert_eq!(vec![0., 0.0, 0.], norm_vec(&vec));

    let vec: Vec<f32> = vec![0., 0.0, 1.];
    assert_eq!(vec, norm_vec(&vec));

    let vec: Vec<f32> = vec![0.33333334, 0.0, 0.6666667];
    assert_eq!(vec, norm_vec(&vec));
}

pub fn apply_weights(pref_matrix_t: &mut [Arr], weights: &[Fl]) {
    for (i, col) in pref_matrix_t.iter_mut().enumerate() {
        if weights[i] != 0. {
            col.iter_mut().for_each(|x| *x *= weights[i]);
        }
    }
}

#[test]
fn test_apply_weights() {
    let mut mat: Mat = vec![vec![0., 1., 0.5], vec![1., 0., 0.5]];
    let weights: Arr = vec![1., 2.];

    apply_weights(&mut mat, &weights);
    let exp: Mat = vec![vec![0.0, 1.0, 0.5], vec![2.0, 0.0, 1.0]];
    assert_eq!(exp, mat);
}

pub fn sum_axis_0(matrix: &[Arr]) -> Arr {
    matrix.iter().map(|x: &Arr| x.iter().sum()).collect()
}

#[test]
fn test_sum_axis_0() {
    let v = vec![
        vec![0., 1.],
        vec![1., 1.],
        vec![2., 1.],
        vec![3., 1.],
        vec![4., 1.],
    ];

    let res: Vec<f32> = v.iter().map(|x: &Vec<f32>| x.iter().sum()).collect();
    let exp: Vec<f32> = vec![1.0, 2., 3., 4., 5.];

    assert_eq!(exp, res);
}

pub struct PromI {
    pub phi_plus_score: Arr,
    pub phi_minus_score: Arr,
    pub phi_plus_matrix: Mat,
    pub phi_minus_matrix: Mat,
}

pub fn prom_i(
    matrix_t: &[Arr],
    weights: &[Fl],
    criteria_type: &[i8],
    func: &[&str],
    q: &[Fl],
    p: &[Fl],
) -> PromI {
    // TODO: store these
    let (mut pref_matrix_plus_t, mut pref_matrix_minus_t) =
        multicriterion_flow(&matrix_t, &weights, &criteria_type, &func, &q, &p);

    apply_weights(&mut pref_matrix_plus_t, &weights);
    apply_weights(&mut pref_matrix_minus_t, &weights);

    PromI {
        phi_plus_score: sum_axis_0(&pref_matrix_plus_t),
        phi_minus_score: sum_axis_0(&pref_matrix_minus_t),
        phi_plus_matrix: pref_matrix_plus_t,
        phi_minus_matrix: pref_matrix_minus_t,
    }
}

fn diff(vec_a: Vec<Fl>, vec_b: Vec<Fl>) -> Vec<Fl> {
    vec_a.into_iter().zip(vec_b).map(|(a, b)| a - b).collect()
}

pub struct PromII {
    pub score: Arr,
    pub weighted_flow: Mat,
}

pub fn prom_ii(p: PromI) -> PromII {
    let score: Arr = diff(p.phi_plus_score, p.phi_minus_score);
    let weighted_flow: Mat = p
        .phi_plus_matrix
        .into_iter()
        .zip(p.phi_minus_matrix)
        .map(|(a, b)| diff(a, b))
        .collect();

    PromII {
        score,
        weighted_flow,
    }
}
