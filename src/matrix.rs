use crate::types::*;

pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

pub fn diff(vec_a: &[Fl], vec_b: &[Fl]) -> Vec<Fl> {
    // TODO: remove by using ndarray
    vec_a.into_iter().zip(vec_b).map(|(a, b)| a - b).collect()
}

#[test]
fn test_diff() {
    let vec: Arr = vec![1., 0.0, 2.0];
    let exp: Fl = 0.;

    assert_eq!(exp, diff(&vec, &vec).iter().sum());
}

pub fn sum_axis_0(matrix: &[Arr]) -> Arr {
    matrix.iter().map(|x: &Arr| x.iter().sum()).collect()
}

#[test]
fn test_sum_axis_0() {
    let v: Mat = vec![
        vec![0., 1.],
        vec![1., 1.],
        vec![2., 1.],
        vec![3., 1.],
        vec![4., 1.],
    ];

    let res: Arr = sum_axis_0(&v);
    let exp: Arr = vec![1.0, 2., 3., 4., 5.];

    assert_eq!(exp, res);
}

pub fn normalize_vec(vec: &[Fl]) -> Arr {
    // rescale proportionally so that all values sum to 1.0.
    // all input values must be >= 0.0
    let s: Fl = vec.iter().sum();
    let mut b: Arr = vec.to_vec();
    if s != 0. {
        b.iter_mut().for_each(|x: &mut Fl| *x /= s);
    }
    b
}

#[test]
fn test_normalize_vec() {
    let vec: Arr = vec![1., 0.0, 2.0];
    let exp: Arr = vec![1. / 3., 0.0, 2. / 3.];
    assert_eq!(exp, normalize_vec(&vec));
    let one: Fl = 1.0;
    assert_eq!(one, normalize_vec(&vec).iter().sum());

    let vec: Arr = vec![0., 0.0, 0.0];
    assert_eq!(vec, normalize_vec(&vec));

    let vec: Arr = vec![0., 0.0, 1.];
    assert_eq!(vec, normalize_vec(&vec));
    assert_eq!(one, normalize_vec(&vec).iter().sum());

    let vec: Arr = vec![1. / 3., 0.0, 2. / 3.];
    assert_eq!(vec, normalize_vec(&vec));
    assert_eq!(one, normalize_vec(&vec).iter().sum());

    let vec: Arr = vec![0., 0.0, 0.5];
    assert_eq!(vec![0., 0.0, 1.], normalize_vec(&vec));
    assert_eq!(one, normalize_vec(&vec).iter().sum());
}
