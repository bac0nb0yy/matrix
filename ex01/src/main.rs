mod field;
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

    fn random_testcases_linear_combination<const N: usize>() {
        for _ in 0..NB_TESTCASE_VECTORS {
            let mut rng = rand::thread_rng();

            let mut my_vectors: Vec<Vector<f64, N>> = Vec::new();
            let mut real_vectors: Vec<[f64; N]> = Vec::new();
            let mut coefs: Vec<f64> = Vec::new();

            for _ in 0..N {
                let vector: [f64; N] = (0..N)
                    .map(|_| rng.gen())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                let coef = rng.gen::<f64>();
                my_vectors.push(Vector::new(vector));
                real_vectors.push(vector);
                coefs.push(coef);
            }

            let my_results = Vector::linear_combination(&my_vectors, &coefs);
            let nalgebra_results: [f64; N] =
                real_vectors
                    .iter()
                    .zip(&coefs)
                    .fold([0.0; N], |mut acc, (vector, &coef)| {
                        for (a, &b) in acc.iter_mut().zip(vector.iter()) {
                            *a += coef * b;
                        }
                        acc
                    });

            for (i, &value) in my_results.data().iter().enumerate() {
                assert_abs_diff_eq!(value, nalgebra_results[i], epsilon = THRESHOLD);
            }
        }
    }

    #[test]
    fn vectors_linear_combination_3() {
        random_testcases_linear_combination::<3>();
    }

    #[test]
    fn vectors_linear_combination_5() {
        random_testcases_linear_combination::<5>();
    }

    #[test]
    fn vectors_linear_combination_42() {
        random_testcases_linear_combination::<42>();
    }

    #[test]
    fn vectors_linear_combination_69() {
        random_testcases_linear_combination::<69>();
    }

    fn generate_mismatched_coefs_vectors<const N: usize>() -> (Vec<Vector<f64, N>>, Vec<f64>) {
        let mut rng = rand::thread_rng();
        let num_vectors = N + rng.gen_range(1..=5);

        let mut my_vectors: Vec<Vector<f64, N>> = Vec::new();
        let mut coefs: Vec<f64> = Vec::new();

        for _ in 0..num_vectors {
            let vector: [f64; N] = (0..N)
                .map(|_| rng.gen())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let coef = rng.gen::<f64>();
            my_vectors.push(Vector::new(vector));
            coefs.push(coef);
        }

        for _ in 0..rng.gen_range(1..=5) {
            let vector: [f64; N] = (0..N)
                .map(|_| rng.gen())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            my_vectors.push(Vector::new(vector));
        }

        (my_vectors, coefs)
    }

    #[test]
    fn vector_bad_size_3() {
        for _ in 0..NB_TESTCASE_VECTORS {
            let (my_vectors, coefs) = generate_mismatched_coefs_vectors::<3>();

            let result = catch_unwind(|| {
                Vector::linear_combination(&my_vectors, &coefs);
            });

            assert!(
                result.is_err(),
                "Adding mismatched number of vectors compared to number of coefs should panic"
            );
        }
    }

    #[test]
    fn vector_bad_size_5() {
        for _ in 0..NB_TESTCASE_VECTORS {
            let (my_vectors, coefs) = generate_mismatched_coefs_vectors::<5>();

            let result = catch_unwind(|| {
                Vector::linear_combination(&my_vectors, &coefs);
            });

            assert!(
                result.is_err(),
                "Adding mismatched number of vectors compared to number of coefs should panic"
            );
        }
    }

    #[test]
    fn vector_bad_size_42() {
        for _ in 0..NB_TESTCASE_VECTORS {
            let (my_vectors, coefs) = generate_mismatched_coefs_vectors::<42>();

            let result = catch_unwind(|| {
                Vector::linear_combination(&my_vectors, &coefs);
            });

            assert!(
                result.is_err(),
                "Adding mismatched number of vectors compared to number of coefs should panic"
            );
        }
    }

    #[test]
    fn vector_bad_size_69() {
        for _ in 0..NB_TESTCASE_VECTORS {
            let (my_vectors, coefs) = generate_mismatched_coefs_vectors::<69>();

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
