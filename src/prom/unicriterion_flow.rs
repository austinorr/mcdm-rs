use super::pref_functions::*;
use super::types::Fl;
use rayon::prelude::*;

pub fn _unicriterion_flow(
    array: &[Fl],
    plus: &mut [Fl],
    minus: &mut [Fl],
    fname: &str,
    q: &Fl,
    p: &Fl,
) {
    let n: Fl = array.len() as Fl;

    let func = _get_pref_function(fname);

    array
        .par_iter()
        .zip(plus.par_iter_mut())
        .zip(minus.par_iter_mut())
        .for_each(|((v1, pl), mi)| {
            for v2 in array.iter() {
                let diff = v1 - v2;
                let ndiff = -diff;
                *pl += func(&diff, q, p) / (n - 1.0);
                *mi += func(&ndiff, q, p) / (n - 1.0);
            }
        });
}

macro_rules! build_unicriterion_flow_fn {
    ($wrapper_name:ident, $alg:expr ) => {
        #[inline(always)]
        pub fn $wrapper_name(array: &[Fl], plus: &mut [Fl], minus: &mut [Fl], q: &Fl, p: &Fl) {
            let n: Fl = array.len() as Fl;

            array
                .par_iter()
                .zip(plus.par_iter_mut())
                .zip(minus.par_iter_mut())
                .for_each(|((v1, pl), mi)| {
                    for v2 in array.iter() {
                        let diff = v1 - v2;
                        let ndiff = -diff;
                        *pl += $alg(&diff, q, p) / (n - 1.0);
                        *mi += $alg(&ndiff, q, p) / (n - 1.0);
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

    macro_rules! parametrize_unicriterion_flow {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;

                let (array, func, q, p) = input;

                let mut plus: Vec<Fl> = vec![0.0; array.len()];
                let mut minus: Vec<Fl> = vec![0.0; array.len()];
                _unicriterion_flow(&array, &mut plus, &mut minus, &func, &q, &p);
                assert_eq!(expected, (plus, minus));
            }
        )*
        }
    }

    parametrize_unicriterion_flow! {
        t1: ((vec![0.8, 0.2, 0.5], "usual", 0.0, 0.0), (vec![1., 0., 0.5], vec![0., 1., 0.5])),
        t2: ((vec![1.,1.,1.], "usual", 0.0, 0.0), (vec![0.,0.,0.],vec![0.,0.,0.])),
        t3: ((vec![0.,0.,0.], "usual", 0.0, 0.0), (vec![0.,0.,0.], vec![0.,0.,0.])),
    }

    #[test]
    fn test_ushape() {
        use is_close::all_close;
        let (ep, em) = (vec![0.5, 0., 0.], vec![0., 0.5, 0.]);
        let (mut pp, mut pm) = (vec![0.; 3usize], vec![0.; 3usize]);
        unicriterion_flow_ushape(&vec![0.8, 0.2, 0.5], &mut pp, &mut pm, &0.4, &0.8);

        assert!(all_close!(ep, pp));
        assert!(all_close!(em, pm));
    }
}
