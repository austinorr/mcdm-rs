use crate::types::*;

pub fn usual(d: &Fl, _q: &Fl, _p: &Fl) -> Fl {
    if d > &0.0 {
        1.0
    } else {
        0.0
    }
}

pub fn ushape(d: &Fl, q: &Fl, _p: &Fl) -> Fl {
    if d > q {
        1.0
    } else {
        0.0
    }
}

pub fn vshape(d: &Fl, _q: &Fl, p: &Fl) -> Fl {
    if (d > &0.0) & (d <= p) {
        d / p
    } else if d > p {
        1.0
    } else {
        0.0
    }
}

pub fn vshape2(d: &Fl, q: &Fl, p: &Fl) -> Fl {
    if (q < d) & (d <= p) {
        (d - q) / (p - q)
    } else if d > p {
        1.0
    } else {
        0.0
    }
}

pub fn level(d: &Fl, q: &Fl, p: &Fl) -> Fl {
    if (q < d) & (d <= p) {
        0.5
    } else if d > p {
        1.0
    } else {
        0.0
    }
}

pub fn get_pref_function(name: &str) -> FPref {
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
        let func_name: &str = "usual";

        let func = get_pref_function(func_name);

        assert_eq!(func(&0.0, &0.0, &0.0), 0.0);
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
    }
}
