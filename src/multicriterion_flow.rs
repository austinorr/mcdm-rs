use crate::types::*;
use crate::unicriterion_flow::unicriterion_flow;

pub fn multicriterion_flow(
    matrix_t: &[Arr],
    pref_functions: &[String],
    q: &[Fl],
    p: &[Fl],
) -> (Mat, Mat) {
    // assume func, q, p are of same length as matrix_t

    let m: usize = matrix_t.len();
    let n: usize = matrix_t[0].len();

    let mut pref_matrix_plus_t: Mat = vec![vec![0.0; n]; m];
    let mut pref_matrix_minus_t: Mat = vec![vec![0.0; n]; m];

    // TODO: parallelize this outer loop.
    for (i, col) in matrix_t.iter().enumerate() {
        // modify preference matrices in place
        unicriterion_flow(
            &col,
            &mut pref_matrix_plus_t[i],
            &mut pref_matrix_minus_t[i],
            &pref_functions[i],
            &q[i],
            &p[i],
        );
    }

    (pref_matrix_plus_t, pref_matrix_minus_t)
}

#[cfg(test)]
mod test {
    use super::*;
    use is_close::all_close;

    macro_rules! parametrize_multicriterion_flow {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                let (array, func_names, q, p) = input;
                let (exp_plus, exp_minus) = expected;
                let (plus, minus) = multicriterion_flow(&array, &func_names, &q, &p);

                for (i, exp) in exp_plus.iter().enumerate() {
                    assert!(all_close!(exp.clone(), plus[i].clone(), rel_tol=1e-6));
                }
                for (i, exp) in exp_minus.iter().enumerate() {
                    assert!(all_close!(exp.clone(), minus[i].clone(), rel_tol=1e-6));
                }
            }
        )*
        }
    }

    parametrize_multicriterion_flow! {
        t1: (
            // input
            (
                vec![vec![0.8, 0.2, 0.5], vec![0.8, 0.2, 0.5]], // array
                vec!["usual".to_string(), "usual".to_string()], // func
                vec![0., 0.], // q
                vec![0., 0.], // p
            ),
            // expected
            (vec![
                vec![1. , 0. , 0.5],
                vec![1. , 0. , 0.5]
                ],
            vec![
                vec![0. , 1. , 0.5],
                vec![0. , 1. , 0.5]
                ]
            )
        ),
        t2: (
            // input
            (
                vec![vec![1.,1.,1.], vec![1.,1.,1.]], // array
                vec!["usual".to_string(), "usual".to_string()], // func
                vec![0., 0.], // q
                vec![0., 0.], // p
            ),
            // expected
            (vec![
                vec![0.,0.,0.],
                vec![0.,0.,0.]
                ],
            vec![
                vec![0.,0.,0.],
                vec![0.,0.,0.]
                ]
            )
        ),
        t3: (
            // input
            (
                vec![vec![0.8, 0.2, 0.5], vec![0.5, 0.8, 0.2]], // array
                vec!["usual".to_string(), "usual".to_string()], // func
                vec![0., 0.], // q
                vec![0., 0.], // p
            ),
            // expected
            (vec![
                vec![1. , 0. , 0.5],
                vec![0.5, 1. , 0. ]
                ],
            vec![
                vec![0. , 1. , 0.5],
                vec![0.5, 0. , 1. ]
                ]
            )
        ),
        t4: (
            // input
            (
                vec![vec![0.8, 0.2, 0.5], vec![0.8, 0.2, 0.5]], // array
                vec!["usual".to_string(), "vshape2".to_string()], // func
                vec![0.01, 0.2], // q
                vec![0.1, 0.9], // p
            ),
            // expected
            (vec![
                vec![1.        , 0.        , 0.5       ],
                vec![0.35714287, 0.        , 0.07142857]
                ],
            vec![
                vec![0.        , 1.        , 0.5       ],
                vec![0.        , 0.35714287, 0.07142857]
                ]
            )
        ),
    }
}
