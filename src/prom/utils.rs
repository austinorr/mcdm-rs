use super::types::{Fl, FromVec2, Mat};
use super::Prom;
use ndarray::{Array1, Array2};
use rand::{distributions::Uniform, Rng};

pub fn generate_prom(n: usize, m: usize) -> Prom {
    let mut rng = rand::thread_rng();
    let range: Uniform<Fl> = Uniform::new(0.0, 20.0);

    let mut matrix_tv: Mat = Vec::new();
    for _ in 0..m {
        matrix_tv.push((0..n).map(|_| rng.sample(range)).collect())
    }

    let matrix_t = Array2::<Fl>::from_vec2(matrix_tv);

    let len: usize = matrix_t.dim().0;
    // you are here

    Prom::new(
        matrix_t,
        Array1::<Fl>::from(vec![1.; len]),
        Array1::<Fl>::from(vec![1.; len]),
        Array1::<String>::from(vec!["usual".to_string(); len]),
        Array1::<Fl>::from(vec![0.; len]),
        Array1::<Fl>::from(vec![0.; len]),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_prom() {
        let mut p = generate_prom(10, 3);
        println!("{:#?}", p);
        p.compute_prom_ii().expect("unable to compute promII");
    }
}
