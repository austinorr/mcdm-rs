use super::types::{Arr, Fl, Mat};
use super::unicriterion_flow::{
    unicriterion_flow_level, unicriterion_flow_ushape, unicriterion_flow_usual,
    unicriterion_flow_vshape, unicriterion_flow_vshape2,
};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

pub fn multicriterion_flow(
    matrix_t: &[Arr],
    pref_function: &[String],
    q: &[Fl],
    p: &[Fl],
) -> (Mat, Mat) {
    let m: usize = matrix_t.len();
    assert!(
        m == pref_function.len() && m == q.len() && m == p.len(),
        "Inputs must be of same length"
    );
    let n: usize = matrix_t[0].len();
    let mut pref_matrix_plus_t: Mat = vec![vec![0.0; n]; m];
    let mut pref_matrix_minus_t: Mat = vec![vec![0.0; n]; m];

    (
        matrix_t,
        &mut pref_matrix_plus_t,
        &mut pref_matrix_minus_t,
        pref_function,
        q,
        p,
    )
        .into_par_iter()
        .for_each(|(col, ppt, pmt, pref, q, p)| {
            // modify preference matrices in place
            match pref.as_str() {
                "usual" => unicriterion_flow_usual(col, ppt, pmt, q, p),
                "ushape" => unicriterion_flow_ushape(col, ppt, pmt, q, p),
                "vshape" => unicriterion_flow_vshape(col, ppt, pmt, q, p),
                "vshape2" => unicriterion_flow_vshape2(col, ppt, pmt, q, p),
                "vshape_2" => unicriterion_flow_vshape2(col, ppt, pmt, q, p),
                "linear" => unicriterion_flow_vshape2(col, ppt, pmt, q, p),
                "level" => unicriterion_flow_level(col, ppt, pmt, q, p),
                _ => panic!("invalid preference function: {:?}", pref),
            }
        });

    (pref_matrix_plus_t, pref_matrix_minus_t)
}

#[cfg(test)]
mod test {
    use super::*;
    use is_close::all_close;

    #[test]
    #[should_panic(expected = "invalid preference function")]
    fn test_mc_panic() {
        let array = vec![vec![0.8, 0.2, 0.5], vec![0.5, 0.8, 0.2]]; // array
        let func_names = vec!["usual".to_string(), "panic!".to_string()]; // func
        let q = vec![0., 0.]; // q
        let p = vec![0., 0.]; // p

        _ = multicriterion_flow(&array, &func_names, &q, &p);
    }

    #[test]
    #[should_panic(expected = "must be of same length")]
    fn test_input_length() {
        let array = vec![
            vec![0.0; 3usize],
            vec![0.0; 3usize],
            vec![0.0; 3usize], // <- extra !
        ]; // array
        let func_names = vec!["usual".to_string(), "usual".to_string()]; // func
        let q = vec![0., 0.]; // q
        let p = vec![0., 0.]; // p

        _ = multicriterion_flow(&array, &func_names, &q, &p);
    }

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
                    let pass:bool = all_close!(exp.clone(), plus[i].clone(), rel_tol=1e-6, abs_tol=1e-3);
                    assert!(pass, "plus: {:#?} == {:#?}", exp.clone(), plus[i].clone());
                }
                for (i, exp) in exp_minus.iter().enumerate() {
                    let pass:bool = all_close!(exp.clone(), minus[i].clone(), rel_tol=1e-6, abs_tol=1e-3);
                    assert!(pass, "minus: {:#?} == {:#?}", exp.clone(), minus[i].clone());
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
        t_usual:(
            // input
            (
                vec![
                    vec![0.8, 0.2, 0.05],
                    vec![0.1, 0.6, 0.4]
                    ], // array
                vec!["usual".to_string(), "usual".to_string()], // func
                vec![0.01, 0.2], // q
                vec![0.1, 0.9], // p
            ),
            // expected
            (vec![
                vec![1.,  0.5, 0. ],
                vec![0. , 1. , 0.5]],

            vec![
                vec![0.,  0.5, 1. ],
                vec![1. , 0. , 0.5]]
            )
        ),
        t_ushape:(
            // input
            (
                vec![vec![0.8, 0.2, 0.05], vec![0.1, 0.6, 0.4]], // array
                vec!["ushape".to_string(), "ushape".to_string()], // func
                vec![0.01, 0.2], // q
                vec![0.1, 0.9], // p
            ),
            // expected
            (vec![
                vec![1.,  0.5, 0. ],
                vec![0. , 0.5, 0.5]],

            vec![
                vec![0.,  0.5, 1. ],
                vec![1. , 0. , 0. ]]
            )
        ),
        t_vshape:(
            // input
            (
                vec![vec![0.8, 0.2, 0.05], vec![0.1, 0.6, 0.4]], // array
                vec!["vshape".to_string(), "vshape".to_string()], // func
                vec![0.01, 0.2], // q
                vec![0.1, 0.9], // p
            ),
            // expected
            (vec![
                vec![1.        , 0.5       , 0.        ],
                vec![0.        , 0.3888889 , 0.16666667]
                ],
            vec![
                vec![0.        , 0.5       , 1.        ],
                vec![0.44444448, 0.        , 0.11111111]]
            )
        ),
        t_vshape2:(
            // input
            (
                vec![vec![0.8, 0.2, 0.05], vec![0.1, 0.6, 0.4]], // array
                vec!["vshape2".to_string(), "vshape2".to_string()], // func
                vec![0.01, 0.2], // q
                vec![0.1, 0.9], // p
            ),
            // expected
            (vec![
                vec![1.        , 0.5       , 0.        ],
                vec![0.        , 0.21428575, 0.07142858]
                ],
            vec![
                vec![0.        , 0.5       , 1.        ],
                vec![0.28571433, 0.0       , 0.        ]]
            )
        ),
        t_level:(
            // input
            (
                vec![vec![0.8, 0.2, 0.05], vec![0.1, 0.6, 0.4]], // array
                vec!["level".to_string(), "level".to_string()], // func
                vec![0.01, 0.2], // q
                vec![0.1, 0.9], // p
            ),
            // expected
            (vec![
                vec![1.  , 0.5 , 0.  ],
                vec![0.  , 0.25 , 0.25]
                ],
            vec![
                vec![0.  , 0.5 , 1.  ],
                vec![0.5 , 0.  , 0.  ]
            ]
            )
        ),

    }
}
