mod field;
mod matrix;
mod vector;

use matrix::Matrix;
use vector::Vector;

#[cfg(test)]
mod vectors {
    use super::*;
    use approx::assert_abs_diff_eq;
    use nalgebra::SVector;
    use rand::prelude::*;

    const NB_TESTCASE_VECTORS: usize = 100;
    const THRESHOLD: f64 = 1e-10;

    fn generate_random_vector<const N: usize>() -> Vector<f64, N> {
        let mut rng = rand::thread_rng();
        let vector: [f64; N] = (0..N)
            .map(|_| rng.gen())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Vector::new(vector)
    }

    fn random_testcases_vectors<const N: usize>(operators: bool) {
        for _ in 0..NB_TESTCASE_VECTORS {
            let mut my_vector: Vector<f64, N> = generate_random_vector::<N>();
            let mut nalgebra_vector: SVector<f64, N> = SVector::from_vec(Vec::from(&*my_vector));
            let mut rng = rand::thread_rng();
            let operation = rng.gen_range(0..4);

            match operation {
                0 => {
                    let random_vector: Vector<f64, N> = generate_random_vector::<N>();
                    let na_random_vector: SVector<f64, N> =
                        SVector::from_vec(random_vector.to_vec());

                    if operators {
                        my_vector += random_vector
                    } else {
                        my_vector.add(&random_vector)
                    };
                    nalgebra_vector += na_random_vector;
                }
                1 => {
                    let random_vector: Vector<f64, N> = generate_random_vector::<N>();
                    let na_random_vector: SVector<f64, N> =
                        SVector::from_vec(random_vector.to_vec());

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
                3 => {
                    let scalar: f64 = rng.gen();

                    if operators {
                        my_vector /= scalar
                    } else {
                        my_vector.inv_scl(scalar)
                    };
                    nalgebra_vector /= scalar;
                }
                _ => unreachable!(),
            }

            for (i, &value) in my_vector.iter().enumerate() {
                assert_abs_diff_eq!(value, nalgebra_vector[i], epsilon = THRESHOLD);
            }
        }
    }

    #[test]
    fn vectors_ops_3() {
        random_testcases_vectors::<3>(false);
        random_testcases_vectors::<3>(true);
    }

    #[test]
    fn vectors_ops_5() {
        random_testcases_vectors::<5>(false);
        random_testcases_vectors::<5>(true);
    }

    #[test]
    fn vectors_ops_42() {
        random_testcases_vectors::<42>(false);
        random_testcases_vectors::<42>(true);
    }

    #[test]
    fn vectors_ops_69() {
        random_testcases_vectors::<69>(false);
        random_testcases_vectors::<69>(true);
    }
}

#[cfg(test)]
mod matrices {
    use super::*;
    use approx::assert_abs_diff_eq;
    use nalgebra::SMatrix;
    use rand::prelude::*;

    const NB_TESTCASE_MATRICES: usize = 100;
    const THRESHOLD: f64 = 1e-10;

    fn generate_random_matrix<const M: usize, const N: usize>() -> Matrix<f64, M, N> {
        let mut rng = rand::thread_rng();
        let data: [[f64; N]; M] = {
            let mut data = [[0.0; N]; M];
            for row in &mut data {
                for elem in row {
                    *elem = rng.gen();
                }
            }
            data
        };
        Matrix::new(data)
    }

    fn flatten<T: Copy, const M: usize, const N: usize>(matrix: &[[T; N]; M]) -> Vec<T> {
        matrix.iter().flatten().copied().collect()
    }

    fn random_testcases_matrices<const M: usize, const N: usize>(operators: bool) {
        for _ in 0..NB_TESTCASE_MATRICES {
            let mut my_matrix: Matrix<f64, M, N> = generate_random_matrix();
            let data_flat = flatten(&my_matrix);
            let mut nalgebra_matrix: SMatrix<f64, M, N> = SMatrix::from_vec(data_flat.clone());
            let mut rng = rand::thread_rng();
            let operation = rng.gen_range(0..4);

            match operation {
                0 => {
                    let random_matrix: Matrix<f64, M, N> = generate_random_matrix();
                    let na_random_matrix: SMatrix<f64, M, N> =
                        SMatrix::from_vec(flatten(&random_matrix));

                    if operators {
                        my_matrix += random_matrix
                    } else {
                        my_matrix.add(&random_matrix)
                    };
                    nalgebra_matrix += na_random_matrix;
                }
                1 => {
                    let random_matrix: Matrix<f64, M, N> = generate_random_matrix();
                    let na_random_matrix: SMatrix<f64, M, N> =
                        SMatrix::from_vec(flatten(&random_matrix));

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
                3 => {
                    let scalar: f64 = rng.gen();

                    if operators {
                        my_matrix /= scalar
                    } else {
                        my_matrix.inv_scl(scalar)
                    };
                    nalgebra_matrix /= scalar;
                }
                _ => unreachable!(),
            }

            for (i, &value) in flatten(&my_matrix).iter().enumerate() {
                assert_abs_diff_eq!(value, nalgebra_matrix[i], epsilon = THRESHOLD);
            }
        }
    }

    #[test]
    fn matrices_ops_3x2() {
        random_testcases_matrices::<3, 2>(false);
        random_testcases_matrices::<3, 2>(true);
    }

    #[test]
    fn matrices_ops_5x4() {
        random_testcases_matrices::<5, 4>(false);
        random_testcases_matrices::<5, 4>(true);
    }

    #[test]
    fn matrices_ops_6x9() {
        random_testcases_matrices::<6, 9>(false);
        random_testcases_matrices::<6, 9>(true);
    }

    #[test]
    fn matrices_ops_2x3() {
        random_testcases_matrices::<2, 3>(false);
        random_testcases_matrices::<2, 3>(true);
    }

    #[test]
    fn matrices_ops_4x5() {
        random_testcases_matrices::<4, 5>(false);
        random_testcases_matrices::<4, 5>(true);
    }

    #[test]
    fn matrices_ops_9x6() {
        random_testcases_matrices::<9, 6>(false);
        random_testcases_matrices::<9, 6>(true);
    }
}

fn main() {
    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    u.add(&v);
    println!("{}", u);
    assert_eq!(u, Vector::from([7., 10.]));

    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    u.sub(&v);
    println!("{}", u);
    assert_eq!(u, Vector::from([-3., -4.]));

    let mut u = Vector::from([2., 3.]);
    u.scl(2.);
    println!("{}", u);
    assert_eq!(u, Vector::from([4., 6.]));

    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    let v = Matrix::from([[7., 4.], [-2., 2.]]);
    u.add(&v);
    println!("{}", u);
    assert_eq!(u, Matrix::from([[8., 6.], [1., 6.]]));

    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    let v = Matrix::from([[7., 4.], [-2., 2.]]);
    u.sub(&v);
    println!("{}", u);
    assert_eq!(u, Matrix::from([[-6., -2.], [5., 2.]]));

    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    u.scl(2.);
    println!("{}", u);
    assert_eq!(u, Matrix::from([[2., 4.], [6., 8.]]));

    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([2., 3.]);
    u += v;
    println!("{}", u);
    assert_eq!(u, Vector::from([4., 6.]));

    u.inv_scl(2.);
    println!("{}", u);
    assert_eq!(u, Vector::from([2., 3.]));

    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    u.inv_scl(2.);
    println!("{}", u);
    assert_eq!(u, Matrix::from([[0.5, 1.], [1.5, 2.]]));

    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    let v = Vector::from([2., 3.]);
    u += v;
    println!("{}", u);
    assert_eq!(u, Matrix::from([[3., 5.], [5., 7.]]));
}
