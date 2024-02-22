use super::types::{Fl, Result};
use super::unicriterion_flow::{
    unicriterion_flow_level, unicriterion_flow_ushape, unicriterion_flow_usual,
    unicriterion_flow_vshape, unicriterion_flow_vshape2,
};
use ndarray::{Array2, ArrayView1, ArrayView2, Axis, Zip};

#[derive(Clone, Debug, Default)]
pub struct MCFlowResult {
    pub pref_matrix_plus_t: Array2<Fl>,
    pub pref_matrix_minus_t: Array2<Fl>,
}

pub fn multicriterion_flow(
    matrix_t: ArrayView2<Fl>,
    pref_function: ArrayView1<String>,
    q: ArrayView1<Fl>,
    p: ArrayView1<Fl>,
) -> Result<MCFlowResult> {
    let (m, n) = matrix_t.dim();
    assert!(
        m == pref_function.len() && m == q.len() && m == p.len(),
        "Inputs must be of same length"
    );
    let mut pref_matrix_plus_t: Array2<Fl> = Array2::zeros((m, n));
    let mut pref_matrix_minus_t: Array2<Fl> = Array2::zeros((m, n));

    Zip::from(matrix_t.axis_iter(Axis(0)))
        .and(pref_matrix_plus_t.axis_iter_mut(Axis(0)))
        .and(pref_matrix_minus_t.axis_iter_mut(Axis(0)))
        .and(pref_function)
        .and(q)
        .and(p)
        .par_for_each(|col, mut ppt, mut pmt, pref, q, p| {
            // modify preference matrices in place
            match pref.as_str() {
                "usual" => unicriterion_flow_usual(col, ppt.view_mut(), pmt.view_mut(), q, p),
                "ushape" => unicriterion_flow_ushape(col, ppt.view_mut(), pmt.view_mut(), q, p),
                "vshape" => unicriterion_flow_vshape(col, ppt.view_mut(), pmt.view_mut(), q, p),
                "vshape2" => unicriterion_flow_vshape2(col, ppt.view_mut(), pmt.view_mut(), q, p),
                "vshape_2" => unicriterion_flow_vshape2(col, ppt.view_mut(), pmt.view_mut(), q, p),
                "linear" => unicriterion_flow_vshape2(col, ppt.view_mut(), pmt.view_mut(), q, p),
                "level" => unicriterion_flow_level(col, ppt.view_mut(), pmt.view_mut(), q, p),
                _ => panic!("invalid preference function: {:?}", pref),
            }
        });

    Ok(MCFlowResult {
        pref_matrix_plus_t,
        pref_matrix_minus_t,
    })
}

impl MCFlowResult {
    pub fn new(
        matrix_t: ArrayView2<Fl>,
        pref_function: ArrayView1<String>,
        q: ArrayView1<Fl>,
        p: ArrayView1<Fl>,
    ) -> Result<MCFlowResult> {
        multicriterion_flow(matrix_t, pref_function, q, p)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use is_close::all_close;
    use ndarray::array;

    #[test]
    #[should_panic(expected = "invalid preference function")]
    fn test_mc_panic() {
        let array = array![[0.8, 0.2, 0.5], [0.5, 0.8, 0.2]]; // array
        let func_names = array!["usual".to_string(), "panic!".to_string()]; // func
        let q = array![0., 0.]; // q
        let p = array![0., 0.]; // p

        _ = multicriterion_flow(array.view(), func_names.view(), q.view(), p.view());
    }

    #[test]
    #[should_panic(expected = "must be of same length")]
    fn test_input_length() {
        let array = array![
            [0.0; 3usize],
            [0.0; 3usize],
            [0.0; 3usize], // <- extra !
        ]; // array
        let func_names = array!["usual".to_string(), "usual".to_string()]; // func
        let q = array![0., 0.]; // q
        let p = array![0., 0.]; // p

        _ = multicriterion_flow(array.view(), func_names.view(), q.view(), p.view());
    }

    macro_rules! parametrize_multicriterion_flow {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() -> Result<()>{
                let (input, expected) = $value;
                let (array, func_names, q, p) = input;
                let (exp_plus, exp_minus) = expected;
                let mc_result = multicriterion_flow(array.view(), func_names.view(), q.view(), p.view())?;
                let plus = mc_result.pref_matrix_plus_t;
                let minus = mc_result.pref_matrix_minus_t;

                for (i, exp) in exp_plus.axis_iter(Axis(0)).enumerate() {
                    let pass:bool = all_close!(exp.to_vec(), plus.index_axis(Axis(0), i).to_vec(), rel_tol=1e-6, abs_tol=1e-3);
                    assert!(pass, "plus: {:#?} == {:#?}", exp, plus.index_axis(Axis(0), i));
                }
                for (i, exp) in exp_minus.axis_iter(Axis(0)).enumerate() {
                    let pass:bool = all_close!(exp.to_vec(), minus.index_axis(Axis(0), i).to_vec(), rel_tol=1e-6, abs_tol=1e-3);
                    assert!(pass, "minus: {:#?} == {:#?}", exp.to_vec(), minus.index_axis(Axis(0), i).to_vec());
                }
                Ok(())
            }
        )*
        }
    }

    parametrize_multicriterion_flow! {
        t1: (
            // input
            (
                array![[0.8, 0.2, 0.5], [0.8, 0.2, 0.5]], // array
                array!["usual".to_string(), "usual".to_string()], // func
                array![0., 0.], // q
                array![0., 0.], // p
            ),
            // expected
            (array![
                [1. , 0. , 0.5],
                [1. , 0. , 0.5]
                ],
            array![
                [0. , 1. , 0.5],
                [0. , 1. , 0.5]
                ]
            )
        ),
        t2: (
            // input
            (
                array![[1.,1.,1.], [1.,1.,1.]], // array
                array!["usual".to_string(), "usual".to_string()], // func
                array![0., 0.], // q
                array![0., 0.], // p
            ),
            // expected
            (array![
                [0.,0.,0.],
                [0.,0.,0.]
                ],
            array![
                [0.,0.,0.],
                [0.,0.,0.]
                ]
            )
        ),
        t3: (
            // input
            (
                array![[0.8, 0.2, 0.5], [0.5, 0.8, 0.2]], // array
                array!["usual".to_string(), "usual".to_string()], // func
                array![0., 0.], // q
                array![0., 0.], // p
            ),
            // expected
            (array![
                [1. , 0. , 0.5],
                [0.5, 1. , 0. ]
                ],
            array![
                [0. , 1. , 0.5],
                [0.5, 0. , 1. ]
                ]
            )
        ),
        t4: (
            // input
            (
                array![[0.8, 0.2, 0.5], [0.8, 0.2, 0.5]], // array
                array!["usual".to_string(), "vshape2".to_string()], // func
                array![0.01, 0.2], // q
                array![0.1, 0.9], // p
            ),
            // expected
            (array![
                [1.        , 0.        , 0.5       ],
                [0.35714287, 0.        , 0.07142857]
                ],
            array![
                [0.        , 1.        , 0.5       ],
                [0.        , 0.35714287, 0.07142857]
                ]
            )
        ),
        t_usual:(
            // input
            (
                array![
                    [0.8, 0.2, 0.05],
                    [0.1, 0.6, 0.4]
                    ], // array
                array!["usual".to_string(), "usual".to_string()], // func
                array![0.01, 0.2], // q
                array![0.1, 0.9], // p
            ),
            // expected
            (array![
                [1.,  0.5, 0. ],
                [0. , 1. , 0.5]],

            array![
                [0.,  0.5, 1. ],
                [1. , 0. , 0.5]]
            )
        ),
        t_ushape:(
            // input
            (
                array![[0.8, 0.2, 0.05], [0.1, 0.6, 0.4]], // array
                array!["ushape".to_string(), "ushape".to_string()], // func
                array![0.01, 0.2], // q
                array![0.1, 0.9], // p
            ),
            // expected
            (array![
                [1.,  0.5, 0. ],
                [0. , 0.5, 0.5]],

            array![
                [0.,  0.5, 1. ],
                [1. , 0. , 0. ]]
            )
        ),
        t_vshape:(
            // input
            (
                array![[0.8, 0.2, 0.05], [0.1, 0.6, 0.4]], // array
                array!["vshape".to_string(), "vshape".to_string()], // func
                array![0.01, 0.2], // q
                array![0.1, 0.9], // p
            ),
            // expected
            (array![
                [1.        , 0.5       , 0.        ],
                [0.        , 0.3888889 , 0.16666667]
                ],
            array![
                [0.        , 0.5       , 1.        ],
                [0.44444448, 0.        , 0.11111111]]
            )
        ),
        t_vshape2:(
            // input
            (
                array![[0.8, 0.2, 0.05], [0.1, 0.6, 0.4]], // array
                array!["vshape2".to_string(), "vshape2".to_string()], // func
                array![0.01, 0.2], // q
                array![0.1, 0.9], // p
            ),
            // expected
            (array![
                [1.        , 0.5       , 0.        ],
                [0.        , 0.21428575, 0.07142858]
                ],
            array![
                [0.        , 0.5       , 1.        ],
                [0.28571433, 0.0       , 0.        ]]
            )
        ),
        t_level:(
            // input
            (
                array![[0.8, 0.2, 0.05], [0.1, 0.6, 0.4]], // array
                array!["level".to_string(), "level".to_string()], // func
                array![0.01, 0.2], // q
                array![0.1, 0.9], // p
            ),
            // expected
            (array![
                [1.  , 0.5 , 0.  ],
                [0.  , 0.25 , 0.25]
                ],
            array![
                [0.  , 0.5 , 1.  ],
                [0.5 , 0.  , 0.  ]
            ]
            )
        ),

    }
}
