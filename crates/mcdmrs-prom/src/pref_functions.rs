use super::types::{FPref, Fl};

const REL_TOL: Fl = 1e-7;

#[inline(always)]
fn lhs(a: &Fl, b: &Fl) -> Fl {
    (a - b).abs()
}

#[inline(always)]
fn rhs(a: &Fl, b: &Fl) -> Fl {
    REL_TOL * a.abs().max(b.abs())
}

#[inline(always)]
fn f_isclose(a: &Fl, b: &Fl) -> bool {
    lhs(a, b) <= rhs(a, b)
}

#[inline(always)]
fn gt(a: &Fl, b: &Fl) -> bool {
    !f_isclose(a, b) && (a > b)
}

#[inline(always)]
fn lt(a: &Fl, b: &Fl) -> bool {
    !f_isclose(a, b) && (a < b)
}

#[allow(dead_code)]
#[inline(always)]
fn ge(a: &Fl, b: &Fl) -> bool {
    f_isclose(a, b) || (a > b)
}

#[inline(always)]
fn le(a: &Fl, b: &Fl) -> bool {
    f_isclose(a, b) || (a < b)
}

#[inline(always)]
pub fn usual(d: &Fl, _q: &Fl, _p: &Fl) -> Fl {
    if gt(d, &0.0) {
        1.0
    } else {
        0.0
    }
}

#[inline(always)]
pub fn ushape(d: &Fl, q: &Fl, _p: &Fl) -> Fl {
    if gt(d, q) {
        1.0
    } else {
        0.0
    }
}

#[inline(always)]
pub fn vshape(d: &Fl, _q: &Fl, p: &Fl) -> Fl {
    if gt(d, &0.0) && le(d, p) {
        d / p
    } else if gt(d, p) {
        1.0
    } else {
        0.0
    }
}

#[inline(always)]
pub fn vshape2(d: &Fl, q: &Fl, p: &Fl) -> Fl {
    if lt(q, d) && le(d, p) {
        (d - q) / (p - q)
    } else if gt(d, p) {
        1.0
    } else {
        0.0
    }
}

#[inline(always)]
pub fn level(d: &Fl, q: &Fl, p: &Fl) -> Fl {
    if lt(q, d) && le(d, p) {
        0.5
    } else if gt(d, p) {
        1.0
    } else {
        0.0
    }
}

pub fn _get_pref_function(name: &str) -> FPref {
    match name {
        "usual" => usual,
        "ushape" => ushape,
        "vshape" => vshape,
        "vshape2" => vshape2,
        "level" => level,
        _ => usual,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_isclose() {
        assert!(f_isclose(&(0.3 - 0.1), &0.2));
        assert!(gt(&(0.3 + (10.0 * REL_TOL) - 0.1), &0.2));
        assert!(lt(&(0.3 - (10.0 * REL_TOL) - 0.1), &0.2));
        assert!(ge(&(0.3 + (10.0 * REL_TOL) - 0.1), &0.2));
        assert!(ge(&(0.3 - 0.1), &0.2));
        assert!(le(&(0.3 - (10.0 * REL_TOL) - 0.1), &0.2));
        assert!(le(&(0.3 - 0.1), &0.2));
    }

    #[test]
    fn test_func_lookup() {
        assert_eq!(_get_pref_function("usual")(&0.0, &0.0, &0.0), 0.0);
        assert_eq!(_get_pref_function("ushape")(&0.0, &0.0, &0.0), 0.0);
        assert_eq!(_get_pref_function("vshape")(&0.0, &0.0, &0.0), 0.0);
        assert_eq!(_get_pref_function("vshape2")(&0.0, &0.0, &0.0), 0.0);
        assert_eq!(_get_pref_function("level")(&0.0, &0.0, &0.0), 0.0);
    }

    macro_rules! parametrize_pref_functions {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (fname, input, expected) = $value;
                let (d, q, p) = input;
                assert_eq!(expected, fname(&d, &q, &p));
            }
        )*
        }
    }

    parametrize_pref_functions! {
        usual_0: (usual, (0.0, 0.0, 0.0), 0.0),
        usual_1: (usual, (1.0, 0.0, 0.0), 1.0),
        usual_2: (usual, (0.5, 0.0, 0.0), 1.0),
        usual_3: (usual, (-0.5, 0.0, 0.0), 0.0),
        ushape_1: (ushape, (-0.5, 0.5, 0.0), 0.0),
        ushape_2: (ushape, (0., 0.5, 0.0), 0.0),
        ushape_3: (ushape, (0.51, 0.5, 0.0), 1.0),
    }
}
