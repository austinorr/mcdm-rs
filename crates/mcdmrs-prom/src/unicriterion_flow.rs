use super::pref_functions::*;
use super::types::Fl;
use ndarray::{ArrayView1, ArrayViewMut1, Zip};

pub fn _unicriterion_flow(
    array: &ArrayView1<Fl>,
    plus: &mut [Fl],
    minus: &mut [Fl],
    fname: &str,
    q: &Fl,
    p: &Fl,
) {
    let n: Fl = array.len() as Fl - 1.0;

    let func = _get_pref_function(fname);

    Zip::from(array)
        .and(plus)
        .and(minus)
        .par_for_each(|&v1, pl, mi| {
            for v2 in array.iter() {
                let diff = v1 - v2;
                let ndiff = -diff;
                *pl += func(&diff, q, p) / n;
                *mi += func(&ndiff, q, p) / n;
            }
        });
}

macro_rules! build_unicriterion_flow_fn {
    ($wrapper_name:ident, $alg:expr ) => {
        pub fn $wrapper_name(
            array: ArrayView1<Fl>,
            plus: ArrayViewMut1<Fl>,
            minus: ArrayViewMut1<Fl>,
            q: &Fl,
            p: &Fl,
        ) {
            // when built with rayon this optimizes using loop unrolling. When built without
            // rayon, this optimizes into 4 lane SIMD.
            // SIMD alone (without parallelism) results in a 400% drop in performance for the
            // benchmark.
            // SIMD alone (without parallelism) results in a 70% drop in performance for the
            // multicriteria benchmark.
            let n: Fl = array.len() as Fl - 1.0;

            Zip::from(array)
                .and(plus)
                .and(minus)
                .par_for_each(|&v1, pl, mi| {
                    for v2 in array.iter() {
                        let diff = v1 - v2;
                        let ndiff = -diff;
                        *pl += $alg(&diff, q, p) / n;
                        *mi += $alg(&ndiff, q, p) / n;
                    }
                });
        }
    };
}

build_unicriterion_flow_fn!(unicriterion_flow_usual, usual);
build_unicriterion_flow_fn!(unicriterion_flow_ushape, ushape);
build_unicriterion_flow_fn!(unicriterion_flow_vshape, vshape);
build_unicriterion_flow_fn!(unicriterion_flow_vshape2, vshape2);
build_unicriterion_flow_fn!(unicriterion_flow_level, level);

#[cfg(test)]
mod test {
    use super::*;
    use ndarray::Array1;

    macro_rules! parametrize_unicriterion_flow {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;

                let (array, func, q, p) = input;

                let mut plus = Array1::<Fl>::from_vec(vec![0.0; array.len()]);
                let mut minus = Array1::<Fl>::from_vec(vec![0.0; array.len()]);
                _unicriterion_flow(
                    &Array1::<Fl>::from_vec(array).view(),
                    &mut plus.as_slice_mut().unwrap(),
                    &mut minus.as_slice_mut().unwrap(),
                    &func,
                    &q,
                    &p,
                );
                assert_eq!(expected, (plus.to_vec(), minus.to_vec()));
            }
        )*
        }
    }

    parametrize_unicriterion_flow! {
        _unicriterion_usual1: ((vec![0.8, 0.2, 0.5], "usual", 0.0, 0.0), (vec![1., 0., 0.5], vec![0., 1., 0.5])),
        _unicriterion_usual2: ((vec![1.,1.,1.], "usual", 0.0, 0.0), (vec![0.,0.,0.],vec![0.,0.,0.])),
        _unicriterion_usual3: ((vec![0.,0.,0.], "usual", 0.0, 0.0), (vec![0.,0.,0.], vec![0.,0.,0.])),
    }

    #[test]
    fn test_ushape() {
        use is_close::all_close;
        let (ep, em) = (vec![0.5, 0., 0.], vec![0., 0.5, 0.]);
        let (mut pp, mut pm) = (Array1::<Fl>::zeros(3), Array1::<Fl>::zeros(3));
        unicriterion_flow_ushape(
            Array1::from_vec(vec![0.8, 0.2, 0.5]).view(),
            pp.view_mut(),
            pm.view_mut(),
            &0.4,
            &0.8,
        );

        assert!(all_close!(ep.clone(), pp));
        assert!(all_close!(em.clone(), pm));

        let (mut pp, mut pm) = (Array1::<Fl>::zeros(3), Array1::<Fl>::zeros(3));

        unicriterion_flow_ushape(
            Array1::from_vec(vec![0.8, 0.2, 0.5]).view(),
            pp.view_mut(),
            pm.view_mut(),
            &0.4,
            &0.8,
        );
        assert!(all_close!(ep.clone(), pp));
        assert!(all_close!(em.clone(), pm));
    }
}
