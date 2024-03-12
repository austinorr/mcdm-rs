use super::cmp::{gt, le, lt};
use super::types::{FPref, Fl};

pub fn usual(d: &Fl, _q: &Fl, _p: &Fl) -> Fl {
    if gt(d, &0.0) {
        1.0
    } else {
        0.0
    }
}

pub fn ushape(d: &Fl, q: &Fl, _p: &Fl) -> Fl {
    if gt(d, q) {
        1.0
    } else {
        0.0
    }
}

pub fn vshape(d: &Fl, _q: &Fl, p: &Fl) -> Fl {
    if gt(d, p) {
        1.0
    } else if gt(d, &0.0) && le(d, p) {
        d / p
    } else {
        0.0
    }
}

pub fn vshape2(d: &Fl, q: &Fl, p: &Fl) -> Fl {
    if gt(d, p) {
        1.0
    } else if lt(q, d) && le(d, p) {
        (d - q) / (p - q)
    } else {
        0.0
    }
}

pub fn level(d: &Fl, q: &Fl, p: &Fl) -> Fl {
    if gt(d, p) {
        1.0
    } else if lt(q, d) && le(d, p) {
        0.5
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
