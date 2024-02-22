use crate::prom::{
    types::{Fl, FromVec2, Mat, Result},
    Criteria, Prom,
};
use ndarray::{Array1, Array2};
use rand::{distributions::Uniform, Rng};

pub fn generate_prom(n: usize, m: usize) -> Result<Prom> {
    let mut rng = rand::thread_rng();
    let range: Uniform<Fl> = Uniform::new(0.0, 20.0);

    let mut matrix_tv: Mat = Vec::new();
    for _ in 0..m {
        matrix_tv.push((0..n).map(|_| rng.sample(range)).collect())
    }

    let matrix_t = Array2::<Fl>::from_vec2(matrix_tv);

    let len: usize = matrix_t.dim().0;
    Prom::new(
        matrix_t,
        Criteria {
            weight: Array1::<Fl>::from(vec![1.; len]),
            criteria_type: Array1::<Fl>::from(vec![1.; len]),
            pref_function: Array1::<String>::from(vec!["usual".to_string(); len]),
            q: Array1::<Fl>::from(vec![0.; len]),
            p: Array1::<Fl>::from(vec![0.; len]),
        },
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_prom() {
        let mut p = generate_prom(10, 3).unwrap();
        println!("{:#?}", p);
        p.compute_prom_ii().expect("unable to compute promII");
    }
}
