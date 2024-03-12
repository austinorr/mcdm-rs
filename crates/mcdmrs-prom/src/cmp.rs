use super::types::Fl;

const REL_TOL: Fl = 1e-7;

fn lhs(a: &Fl, b: &Fl) -> Fl {
    (a - b).abs()
}

fn rhs(a: &Fl, b: &Fl) -> Fl {
    REL_TOL * a.abs().max(b.abs())
}

fn f_isclose(a: &Fl, b: &Fl) -> bool {
    lhs(a, b) <= rhs(a, b)
}

pub fn gt(a: &Fl, b: &Fl) -> bool {
    !f_isclose(a, b) && (a > b)
}

pub fn lt(a: &Fl, b: &Fl) -> bool {
    !f_isclose(a, b) && (a < b)
}

#[allow(dead_code)]
pub fn ge(a: &Fl, b: &Fl) -> bool {
    f_isclose(a, b) || (a > b)
}

pub fn le(a: &Fl, b: &Fl) -> bool {
    f_isclose(a, b) || (a < b)
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
}
