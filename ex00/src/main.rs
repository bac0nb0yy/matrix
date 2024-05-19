mod matrix;
mod vector;

use matrix::Matrix;
use vector::Vector;

#[cfg(test)]
mod vectors {
    use super::*;
    use nalgebra::DVector;
    use rand::prelude::*;
    use std::panic::catch_unwind;

    const NB_TESTCASE_VECTORS: usize = 100;
    const VECTORS_SIZE: usize = 2;
    const THRESHOLD: f64 = 1e-10;

    fn generate_random_vector(size: usize) -> Vector<f64> {
        let mut rng = rand::thread_rng();
        let data: Vec<f64> = (0..size).map(|_| rng.gen()).collect();
        Vector::new(data, Some(size))
    }

    fn random_testcases_vectors(operators: bool) {
        for _ in 0..NB_TESTCASE_VECTORS {
            let mut my_vector: Vector<f64> = generate_random_vector(VECTORS_SIZE);
            let mut nalgebra_vector: DVector<f64> =
                DVector::from_vec(Vec::from(my_vector.get_data().clone()));
            let mut rng = rand::thread_rng();
            let operation = rng.gen_range(0..3);

            match operation {
                0 => {
                    let random_vector: Vector<f64> = generate_random_vector(VECTORS_SIZE);
                    let na_random_vector: DVector<f64> =
                        DVector::from_vec(random_vector.get_data().clone());

                    if operators {
                        my_vector += random_vector
                    } else {
                        my_vector.add(&random_vector)
                    };
                    nalgebra_vector += na_random_vector;
                }
                1 => {
                    let random_vector: Vector<f64> = generate_random_vector(VECTORS_SIZE);
                    let na_random_vector: DVector<f64> =
                        DVector::from_vec(random_vector.get_data().clone());

                    if operators {
                        my_vector -= random_vector
                    } else {
                        my_vector.sub(&random_vector)
                    };
                    nalgebra_vector -= na_random_vector;
                }
                2 => {
                    let scalar: f64 = rng.gen();

                    if operators {
                        my_vector *= scalar
                    } else {
                        my_vector.scl(scalar)
                    };
                    nalgebra_vector *= scalar;
                }
                _ => unreachable!(),
            }

            for (i, &value) in my_vector.get_data().iter().enumerate() {
                assert!(
                    (value - nalgebra_vector[i]).abs() < THRESHOLD,
                    "Mismatch at index {}: {} != {}",
                    i,
                    value,
                    nalgebra_vector[i]
                );
            }
        }
    }

    #[test]
    fn vectors_ops() {
        random_testcases_vectors(false);
        random_testcases_vectors(true);
    }

    fn generate_mismatched_vectors() -> (Vector<f64>, Vector<f64>) {
        let mut rng = rand::thread_rng();
        let size1: usize = rng.gen_range(1..=10);
        let mut size2: usize = rng.gen_range(1..=10);

        while size1 == size2 {
            size2 = rng.gen_range(1..=10);
        }

        let data1: Vec<f64> = (0..size1).map(|_| rng.gen()).collect();
        let data2: Vec<f64> = (0..size2).map(|_| rng.gen()).collect();

        (
            Vector::new(data1, Some(size1)),
            Vector::new(data2, Some(size2)),
        )
    }

    #[test]
    fn vector_bad_size() {
        for _ in 0..NB_TESTCASE_VECTORS {
            let (vector1, vector2) = generate_mismatched_vectors();

            let result = catch_unwind(|| {
                let mut vector1_clone = vector1.clone();
                let vector2_clone = vector2.clone();
                vector1_clone.add(&vector2_clone);
            });
            assert!(result.is_err(), "Adding mismatched vectors should panic");
        }
    }
}

#[cfg(test)]
mod matrices {
    use super::*;
    use nalgebra::DMatrix;
    use rand::prelude::*;
    use std::panic::catch_unwind;

    const NB_TESTCASE_MATRICES: usize = 100;
    const MATRIX_SIZE: usize = 2;
    const THRESHOLD: f64 = 1e-10;

    fn generate_random_matrix(size: usize) -> Matrix<f64> {
        let mut rng = rand::thread_rng();
        let data: Vec<Vec<f64>> = (0..size)
            .map(|_| (0..size).map(|_| rng.gen()).collect())
            .collect();
        Matrix::new(data, Some(size), Some(size))
    }

    fn flatten<T>(matrix: &Vec<Vec<T>>) -> Vec<T>
    where
        T: Clone,
    {
        matrix.iter().flat_map(|row| row.iter().cloned()).collect()
    }

    fn random_testcases_matrices(operators: bool) {
        for _ in 0..NB_TESTCASE_MATRICES {
            let mut my_matrix: Matrix<f64> = generate_random_matrix(MATRIX_SIZE);
            let data_flat = flatten(my_matrix.get_data());
            let mut nalgebra_matrix: DMatrix<f64> =
                DMatrix::from_vec(MATRIX_SIZE, MATRIX_SIZE, data_flat.clone());
            let mut rng = rand::thread_rng();
            let operation = rng.gen_range(0..3);

            match operation {
                0 => {
                    let random_matrix: Matrix<f64> = generate_random_matrix(MATRIX_SIZE);
                    let na_random_matrix: DMatrix<f64> = DMatrix::from_vec(
                        MATRIX_SIZE,
                        MATRIX_SIZE,
                        flatten(random_matrix.get_data()),
                    );

                    if operators {
                        my_matrix += random_matrix
                    } else {
                        my_matrix.add(&random_matrix)
                    };
                    nalgebra_matrix += na_random_matrix;
                }
                1 => {
                    let random_matrix: Matrix<f64> = generate_random_matrix(MATRIX_SIZE);
                    let na_random_matrix: DMatrix<f64> = DMatrix::from_vec(
                        MATRIX_SIZE,
                        MATRIX_SIZE,
                        flatten(random_matrix.get_data()),
                    );

                    if operators {
                        my_matrix -= random_matrix
                    } else {
                        my_matrix.sub(&random_matrix)
                    };
                    nalgebra_matrix -= na_random_matrix;
                }
                2 => {
                    let scalar: f64 = rng.gen();

                    if operators {
                        my_matrix *= scalar
                    } else {
                        my_matrix.scl(scalar)
                    };
                    nalgebra_matrix *= scalar;
                }
                _ => unreachable!(),
            }

            for (i, &value) in flatten(my_matrix.get_data()).iter().enumerate() {
                assert!(
                    (value - nalgebra_matrix[i]).abs() < THRESHOLD,
                    "Mismatch at index {}: {} != {}",
                    i,
                    value,
                    nalgebra_matrix[i]
                );
            }
        }
    }

    fn generate_mismatched_matrices() -> (Matrix<f64>, Matrix<f64>) {
        let mut rng = rand::thread_rng();
        let rows1: usize = rng.gen_range(1..=5);
        let mut rows2: usize = rng.gen_range(1..=5);

        while rows1 == rows2 {
            rows2 = rng.gen_range(1..=5);
        }

        let cols1: usize = rng.gen_range(1..=5);
        let mut cols2: usize = rng.gen_range(1..=5);

        let another_bad_size: usize = rng.gen_range(0..=1);

        println!("{}", another_bad_size);
        if another_bad_size == 0 {
            while cols1 == cols2 {
                cols2 = rng.gen_range(1..=5);
            }
        }

        let data1: Vec<Vec<f64>> = (0..rows1)
            .map(|_| (0..cols1).map(|_| rng.gen()).collect())
            .collect();
        let data2: Vec<Vec<f64>> = (0..rows2)
            .map(|_| (0..cols2).map(|_| rng.gen()).collect())
            .collect();

        (
            Matrix::new(data1.clone(), Some(rows1), Some(cols1)),
            Matrix::new(data2.clone(), Some(rows2), Some(cols2)),
        )
    }

    #[test]
    fn matrices_ops() {
        random_testcases_matrices(false);
        random_testcases_matrices(true);
    }

    #[test]
    fn matrices_bad_size() {
        for _ in 0..NB_TESTCASE_MATRICES {
            let (matrix1, matrix2) = generate_mismatched_matrices();

            let result = catch_unwind(|| {
                let mut matrix1_clone = matrix1.clone();
                let matrix2_clone = matrix2.clone();
                matrix1_clone.add(&matrix2_clone);
            });
            assert!(result.is_err(), "Adding mismatched matrices should panic");
        }
    }
}

fn main() {
    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    u.add(&v);
    println!("{}", u);
    // [7.0]
    // [10.0]
    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    u.sub(&v);
    println!("{}", u);
    // [-3.0]
    // [-4.0]
    let mut u = Vector::from([2., 3.]);
    u.scl(2.);
    println!("{}", u);
    // [4.0]
    // [6.0]
    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    let v = Matrix::from([[7., 4.], [-2., 2.]]);
    u.add(&v);
    println!("{}", u);
    // [8.0, 6.0]
    // [1.0, 6.0]
    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    let v = Matrix::from([[7., 4.], [-2., 2.]]);
    u.sub(&v);
    println!("{}", u);
    // [-6.0, -2.0]
    // [5.0, 2.0]
    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    u.scl(2.);
    println!("{}", u);
    // [2.0, 4.0]
    // [6.0, 8.0]
}
