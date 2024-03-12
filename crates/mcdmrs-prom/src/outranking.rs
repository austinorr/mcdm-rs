use super::cmp::{gt, lt};
use super::types::{Fl, MCDMRSError, Result};

use ndarray::{Array2, ArrayView1};

pub fn comparable(ap: &Fl, am: &Fl, bp: &Fl, bm: &Fl) -> bool {
    // return 1 if comparable
    !((gt(ap, bp) && gt(am, bm)) || (lt(ap, bp) && lt(am, bm)))
}

pub fn outranks(ap: &Fl, am: &Fl, bp: &Fl, bm: &Fl) -> bool {
    gt(&(ap - am), &(bp - bm))
}

pub fn has_link_ab(ap: &Fl, am: &Fl, bp: &Fl, bm: &Fl) -> bool {
    if comparable(ap, am, bp, bm) {
        return outranks(ap, am, bp, bm);
    }
    false
}

pub fn outranking_adjacency_matrix(
    phi_plus: ArrayView1<Fl>,
    phi_minus: ArrayView1<Fl>,
) -> Result<Array2<Fl>> {
    let n = phi_plus.len();
    let is_valid = n == phi_minus.len();

    if !is_valid {
        return Err(MCDMRSError::Error("Inputs must be of same length!".to_string()).into());
    }

    let mut matrix: Array2<Fl> = Array2::zeros((n, n));

    matrix.indexed_iter_mut().for_each(|((i, j), v)| {
        let (ap, am) = (&phi_plus[i], &phi_minus[i]);
        let (bp, bm) = (&phi_plus[j], &phi_minus[j]);
        *v = has_link_ab(ap, am, bp, bm).into();
    });

    Ok(matrix)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::{array, Array1, Axis};

    #[test]
    fn test_incomparable() {
        let inputs: [[[Fl; 2]; 2]; 3] = [
            [[1., 1.], [0., 0.]],
            [[0., 0.], [1., 1.]],
            [[7., 7.], [3., 3.]],
        ];

        for case in inputs {
            let a = case[0];
            let b = case[1];

            let (ap, am, bp, bm) = (a[0], a[1], b[0], b[1]);
            assert!(!comparable(&ap, &am, &bp, &bm));
            assert!(!comparable(&bp, &bm, &ap, &am));
        }
    }

    #[test]
    fn test_outranking_adjacency() {
        let arr = array![
            // phi+, phi-
            [0.3573, 0.1],
            [0.276, 0.2213],
            [0.206, 0.1927],
            [0.256, 0.2573],
            [0.2647, 0.422],
            [0.228, 0.3947],
        ];

        let arrt = arr.t();
        let plus = arrt.index_axis(Axis(0), 0);
        let minus = arrt.index_axis(Axis(0), 1);

        let exp: Array2<Fl> = array![
            [0, 1, 1, 1, 1, 1],
            [0, 0, 0, 1, 1, 1],
            [0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 1],
            [0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0],
        ]
        .mapv(|x| x as Fl);

        let adj = outranking_adjacency_matrix(plus.view(), minus.view()).unwrap();
        let flat: Array1<Fl> = Array1::from_iter(adj.iter()).mapv(|el| *el);
        let exp: Array1<Fl> = Array1::from_iter(exp.iter()).mapv(|el| *el);

        assert_eq!(flat, exp);
    }
}
