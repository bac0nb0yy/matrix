mod vector;

use vector::Vector;

#[cfg(test)]
mod vectors {
    use super::*;
    use approx::assert_abs_diff_eq;
    use rand::prelude::*;
    use std::panic::catch_unwind;

    const NB_TESTCASE_VECTORS: usize = 100;
    const THRESHOLD: f64 = 1e-10;

    fn random_testcases_linear_combination() {
        for _ in 0..NB_TESTCASE_VECTORS {
            let mut rng = rand::thread_rng();
            let num_vectors = rng.gen_range(1..=5);
            let size = rng.gen_range(1..=10);

            let mut my_vectors: Vec<Vector<f64>> = Vec::new();
            let mut real_vectors: Vec<Vec<f64>> = Vec::new();
            let mut coefs: Vec<f64> = Vec::new();

            for _ in 0..num_vectors {
                let vector = Vec::<f64>::from_iter((0..size).map(|_| rng.gen::<f64>()));
                let coef = rng.gen::<f64>();
                my_vectors.push(Vector::new(vector.clone(), Some(size)));
                real_vectors.push(vector);
                coefs.push(coef);
            }

            let my_results = Vector::linear_combination(&my_vectors, &coefs);
            let nalgebra_results: Vec<f64> = real_vectors.iter().zip(&coefs).fold(
                vec![0.0; real_vectors[0].len()],
                |mut acc, (vector, &coef)| {
                    for (a, &b) in acc.iter_mut().zip(vector.iter()) {
                        *a += coef * b;
                    }
                    acc
                },
            );

            for (i, &value) in my_results.get_data().iter().enumerate() {
                assert_abs_diff_eq!(value, nalgebra_results[i], epsilon = THRESHOLD);
            }
        }
    }

    #[test]
    fn vectors_linear_combination() {
        random_testcases_linear_combination();
    }

    fn generate_mismatched_coefs_vectors() -> (Vec<Vector<f64>>, Vec<f64>) {
        let mut rng = rand::thread_rng();
        let num_vectors = rng.gen_range(1..=5);
        let size = rng.gen_range(1..=10);

        let mut my_vectors: Vec<Vector<f64>> = Vec::new();
        let mut coefs: Vec<f64> = Vec::new();

        for _ in 0..num_vectors {
            let vector = Vec::<f64>::from_iter((0..size).map(|_| rng.gen::<f64>()));
            let coef = rng.gen::<f64>();
            my_vectors.push(Vector::new(Vec::from(vector.clone()), Some(size)));
            coefs.push(coef);
        }

        for _ in 0..rng.gen_range(1..=5) {
            let vector = Vec::<f64>::from_iter((0..size).map(|_| rng.gen::<f64>()));
            my_vectors.push(Vector::new(Vec::from(vector.clone()), Some(size)));
        }

        (my_vectors, coefs)
    }

    #[test]
    fn vector_bad_size() {
        for _ in 0..NB_TESTCASE_VECTORS {
            let (my_vectors, coefs) = generate_mismatched_coefs_vectors();

            let result = catch_unwind(|| {
                Vector::linear_combination(&my_vectors, &coefs);
            });

            assert!(
                result.is_err(),
                "Adding mismatched number of vectors compared to number of coefs should panic"
            );
        }
    }
}

fn main() {
    let e1 = Vector::from([1., 0., 0.]);
    let e2 = Vector::from([0., 1., 0.]);
    let e3 = Vector::from([0., 0., 1.]);

    let v1 = Vector::from([1., 2., 3.]);
    let v2 = Vector::from([0., 10., -100.]);

    println!(
        "{}",
        Vector::linear_combination(&[e1, e2, e3], &[10.0, -2.0, 0.5])
    );
    // [10.]
    // [-2.]
    // [0.5]

    println!("{}", Vector::linear_combination(&[v1, v2], &[10., -2.]));
    // [10.]
    // [0.]
    // [230.]
}
