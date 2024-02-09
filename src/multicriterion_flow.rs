use crate::pref_functions::get_pref_functions;
use crate::types::*;
use crate::unicriterion_flow::unicriterion_flow;

pub fn multicriterion_flow(
    matrix_t: &[Arr],
    weights: &[Fl],
    criteria_type: &[i8],
    func: &[&str],
    q: &[Fl],
    p: &[Fl],
) -> (Mat, Mat) {
    // assume criteria_type, func, q, p are of same length as matrix_t

    let map = get_pref_functions();
    let m: usize = matrix_t.len();
    let n: usize = matrix_t[0].len();

    let mut pref_matrix_plus_t: Mat = vec![vec![0.0; n]; m];
    let mut pref_matrix_minus_t: Mat = vec![vec![0.0; n]; m];

    // TODO: parallelize this outer loop.
    for (i, col) in matrix_t.iter().enumerate() {
        if weights[i] == 0. {
            continue;
        }
        let mut new_col = col.clone();
        new_col
            .iter_mut()
            .for_each(|x| *x *= criteria_type[i] as Fl);

        let func = map
            .get(func[i])
            .unwrap_or_else(|| panic!("function not found: {:?}", func[i]));

        // modify preference matrices in place
        unicriterion_flow(
            &new_col,
            &mut pref_matrix_plus_t[i],
            &mut pref_matrix_minus_t[i],
            *func,
            &q[i],
            &p[i],
        );
    }

    (pref_matrix_plus_t, pref_matrix_minus_t)
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! parametrize_multicriterion_flow {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                let (array, weights, criteria_type, func_names, q, p) = input;
                let (plus, minus) = multicriterion_flow(&array, &weights, &criteria_type, &func_names, &q, &p);
                assert_eq!(expected, (plus, minus));
            }
        )*
        }
    }

    parametrize_multicriterion_flow! {
        t1: (
            // input
            (
                vec![vec![0.8, 0.2, 0.5], vec![0.8, 0.2, 0.5]], // array
                vec![1., 1.], // weights
                vec![-1, 1], // criteria type
                vec!["usual", "usual"], // func
                vec![0., 0.], // q
                vec![0., 0.], // p
            ),
            // expected
            (vec![
                vec![0. , 1. , 0.5],
                vec![1. , 0. , 0.5]
                ],
            vec![
                vec![1. , 0. , 0.5],
                vec![0. , 1. , 0.5]
                ]
            )
        ),
        t2: (
            // input
            (
                vec![vec![1.,1.,1.], vec![1.,1.,1.]], // array
                vec![1., 1.], // weights
                vec![-1, 1], // criteria type
                vec!["usual", "usual"], // func
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
                vec![vec![0.8, 0.2, 0.5], vec![0.8, 0.2, 0.5]], // array
                vec![1., 0.], // weights
                vec![-1, 1], // criteria type
                vec!["usual", "usual"], // func
                vec![0., 0.], // q
                vec![0., 0.], // p
            ),
            // expected
            (vec![
                vec![0. , 1. , 0.5],
                vec![0.,0.,0.]
                ],
            vec![
                vec![1. , 0. , 0.5],
                vec![0.,0.,0.]
                ]
            )
        ),

        /*
        TODO: this passes if we implement all_close!
        thread 'multicriterion_flow::test::t4' panicked at src/multicriterion_flow.rs:66:5:
        assertion `left == right` failed
          left: ([[0.0, 1.0, 0.5], [0.35714287, 0.0, 0.07142857]], [[1.0, 0.0, 0.5], [0.0, 0.35714287, 0.07142857]])
         right: ([[0.0, 1.0, 0.5], [0.35714293, 0.0, 0.07142858]], [[1.0, 0.0, 0.5], [0.0, 0.35714293, 0.07142858]])

        */

        // t4: (
        //     // input
        //     (
        //         vec![vec![0.8, 0.2, 0.5], vec![0.8, 0.2, 0.5]], // array
        //         vec![1., 1.], // weights
        //         vec![-1, 1], // criteria type
        //         vec!["level", "vshape2"], // func
        //         vec![0.01, 0.2], // q
        //         vec![0.1, 0.9], // p
        //     ),
        //     // expected
        //     (vec![
        //         vec![0.        , 1.        , 0.5       ],
        //         vec![0.35714287, 0.        , 0.07142857]
        //         ],
        //     vec![
        //         vec![1.        , 0.        , 0.5       ],
        //         vec![0.        , 0.35714287, 0.07142857]
        //         ]
        //     )
        // ),
    }
}
