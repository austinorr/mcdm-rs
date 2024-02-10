use crate::pref_functions::get_pref_function;
use crate::types::*;

pub fn unicriterion_flow(
    array: &[Fl],
    plus: &mut [Fl],
    minus: &mut [Fl],
    fname: &str,
    q: &Fl,
    p: &Fl,
) {
    let n: Fl = array.len() as Fl;
    let mut diff: Fl;
    let mut ndiff: Fl;
    let mut plus_sum: Fl;
    let mut minus_sum: Fl;

    let func = get_pref_function(fname);

    for (i, v1) in array.iter().enumerate() {
        plus_sum = 0.0;
        minus_sum = 0.0;
        for v2 in array.iter() {
            diff = v1 - v2;
            ndiff = -diff;
            plus_sum += func(&diff, q, p);
            minus_sum += func(&ndiff, q, p);
            plus[i] = plus_sum / (n - 1.0);
            minus[i] = minus_sum / (n - 1.0);
        }
    }
}

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
                unicriterion_flow(&array, &mut plus, &mut minus, &func, &q, &p);
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
}
