use super::types::{Fl, Mat};
use super::Prom;
use rand::{distributions::Uniform, Rng};

pub fn generate_prom(n: usize, m: usize) -> Prom {
    let mut rng = rand::thread_rng();
    let range: Uniform<Fl> = Uniform::new(0.0, 20.0);

    let mut matrix_t: Mat = Vec::new();
    for _ in 0..m {
        matrix_t.push((0..n).map(|_| rng.sample(range)).collect())
    }

    let len: usize = matrix_t.len();

    Prom::new(
        matrix_t,
        vec![1.; len],
        vec![1.; len],
        vec!["usual".to_string(); len],
        vec![0.; len],
        vec![0.; len],
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_prom() {
        let mut p = generate_prom(10, 3);
        _ = p.compute_prom_ii().expect("unable to compute promII");
    }
}
