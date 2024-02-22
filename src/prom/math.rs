use super::types::{Fl, Result};
use ndarray::{Array1, Array2, ArrayView1, ArrayView2};

pub fn mult_axis_0(ndarr: ArrayView2<Fl>, other: ArrayView1<Fl>) -> Result<Array2<Fl>> {
    Ok(&ndarr * &other.into_shape((ndarr.dim().0, 1))?)
}

pub fn min_max_norm(array: ArrayView1<Fl>) -> Array1<Fl> {
    let _max: Fl = *array.iter().max_by(|a, b| a.total_cmp(b)).unwrap_or(&0.0);
    let _min: Fl = *array.iter().min_by(|a, b| a.total_cmp(b)).unwrap_or(&0.0);
    let range = _max - _min;
    if range.abs() < 1e-7 {
        Array1::ones(array.len())
    } else {
        (&array - _min) / range
    }
}

pub fn normalize_vec(array: ArrayView1<Fl>) -> Array1<Fl> {
    // rescale proportionally so that all values sum to 1.0.
    let s: Fl = array.iter().sum();

    if s > 1e-5 {
        &array / s
    } else {
        array.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::{array, Axis};

    #[test]
    fn test_transpose() {
        let mat = array![[0., 1., 0.5], [1., 0., 0.5]];
        let new_mat = mat.t().to_owned();
        let exp = array![[0.0, 1.0], [1.0, 0.0], [0.5, 0.5]];

        assert_eq!(exp, new_mat);
    }

    #[test]
    fn test_diff() {
        let vec = array![1., 0.0, 2.0];
        let exp: Fl = 0.;

        assert_eq!(exp, (&vec - &vec).iter().sum::<Fl>());
    }

    #[test]
    fn test_sum_axis_0() {
        let v = array![[0., 1.], [1., 1.], [2., 1.], [3., 1.], [4., 1.],];

        let res = v.sum_axis(Axis(1));
        let exp = array![1.0, 2., 3., 4., 5.];

        assert_eq!(exp, res);
    }

    #[test]
    fn test_apply_weights() {
        let mat = array![[0., 1., 0.5], [1., 0., 0.5]];
        let weights = array![1., 2.];

        let new_mat = mult_axis_0(mat.view(), weights.view()).unwrap();
        let exp = array![[0.0, 1.0, 0.5], [2.0, 0.0, 1.0]];
        assert_eq!(exp, new_mat);
    }

    #[test]
    fn test_normalize_vec() {
        let vec = array![1., 0.0, 2.0];
        let exp = array![1. / 3., 0.0, 2. / 3.];
        assert_eq!(exp, normalize_vec(vec.view()));
        let one: Fl = 1.0;
        assert_eq!(one, normalize_vec(vec.view()).iter().sum::<Fl>());

        let vec = array![0., 0.0, 0.0];
        assert_eq!(&vec, normalize_vec(vec.view()));

        let vec = array![0., 0.0, 1.];
        assert_eq!(vec, normalize_vec(vec.view()));
        assert_eq!(one, normalize_vec(vec.view()).iter().sum::<Fl>());

        let vec = array![1. / 3., 0.0, 2. / 3.];
        assert_eq!(vec, normalize_vec(vec.view()));
        assert_eq!(one, normalize_vec(vec.view()).iter().sum::<Fl>());

        let vec = array![0., 0.0, 0.5];
        assert_eq!(array![0., 0.0, 1.], normalize_vec(vec.view()));
        assert_eq!(one, normalize_vec(vec.view()).iter().sum::<Fl>());
    }
}
