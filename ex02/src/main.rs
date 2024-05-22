mod field;
mod matrix;
mod vector;

use matrix::Matrix;
use vector::Vector;

use std::ops::{Add, Mul, Sub};

fn lerp<T, K>(u: T, v: T, t: K) -> T
where
    T: Add<Output = T> + Sub<Output = T> + Mul<K, Output = T> + Clone,
{
    u.clone() + (v - u) * t
}

#[cfg(test)]
mod vectors {
    use super::*;
    use approx::assert_abs_diff_eq;
    use nalgebra::SVector;
    use rand::prelude::*;

    const NB_TESTCASE_VECTORS: usize = 100;
    const THRESHOLD: f64 = 1e-10;

    fn test_lerp<const N: usize>() {
        for _ in 0..NB_TESTCASE_VECTORS {
            let mut rng = rand::thread_rng();
            let v1: [f64; N] = (0..N)
                .map(|_| rng.gen())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let v2: [f64; N] = (0..N)
                .map(|_| rng.gen())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            let my_v1 = Vector::new(v1.clone());
            let my_v2 = Vector::new(v2.clone());
            let nalgebra_v1 = SVector::<f64, N>::from_vec(v1.to_vec());
            let nalgebra_v2 = SVector::<f64, N>::from_vec(v2.to_vec());

            let t: f64 = rng.gen();

            let my_result = lerp(my_v1, my_v2, t);
            let nalgebra_result = nalgebra_v1.lerp(&nalgebra_v2, t);

            for (i, &value) in my_result.data().iter().enumerate() {
                assert_abs_diff_eq!(value, nalgebra_result[i], epsilon = THRESHOLD);
            }
        }
    }

    #[test]
    fn test_lerp_vector3() {
        test_lerp::<3>();
    }

    #[test]
    fn test_lerp_vector5() {
        test_lerp::<5>();
    }

    #[test]
    fn test_lerp_vector42() {
        test_lerp::<42>();
    }

    #[test]
    fn test_lerp_vector69() {
        test_lerp::<69>();
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

    fn test_lerp_matrices<const M: usize, const N: usize>() {
        for _ in 0..NB_TESTCASE_MATRICES {
            let mut rng = rand::thread_rng();
            let m1: [[f64; N]; M] = (0..M)
                .map(|_| {
                    (0..N)
                        .map(|_| rng.gen())
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let m2: [[f64; N]; M] = (0..M)
                .map(|_| {
                    (0..N)
                        .map(|_| rng.gen())
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            let my_m1 = Matrix::new(m1.clone());
            let my_m2 = Matrix::new(m2.clone());
            let nalgebra_m1 =
                SMatrix::<f64, M, N>::from_vec(m1.iter().flatten().copied().collect());
            let nalgebra_m2 =
                SMatrix::<f64, M, N>::from_vec(m2.iter().flatten().copied().collect());

            let t: f64 = rng.gen();

            let my_result = lerp(my_m1, my_m2, t);
            let nalgebra_result = nalgebra_m1 * (1.0 - t) + nalgebra_m2 * t;

            for (i, &value) in my_result.data().iter().flatten().enumerate() {
                assert_abs_diff_eq!(value, nalgebra_result[i], epsilon = THRESHOLD);
            }
        }
    }

    #[test]
    fn lerp_matrices_3x2() {
        test_lerp_matrices::<3, 2>();
        test_lerp_matrices::<3, 2>();
    }

    #[test]
    fn lerp_matrices_5x4() {
        test_lerp_matrices::<5, 4>();
        test_lerp_matrices::<5, 4>();
    }

    #[test]
    fn lerp_matrices_6x9() {
        test_lerp_matrices::<6, 9>();
        test_lerp_matrices::<6, 9>();
    }

    #[test]
    fn lerp_matrices_2x3() {
        test_lerp_matrices::<2, 3>();
        test_lerp_matrices::<2, 3>();
    }

    #[test]
    fn lerp_matrices_4x5() {
        test_lerp_matrices::<4, 5>();
        test_lerp_matrices::<4, 5>();
    }

    #[test]
    fn lerp_matrices_9x6() {
        test_lerp_matrices::<9, 6>();
        test_lerp_matrices::<9, 6>();
    }
}

fn main() {
    println!("{}", lerp(0., 1., 0.));
    // 0.0
    println!("{}", lerp(0., 1., 1.));
    // 1.0
    println!("{}", lerp(0., 1., 0.5));
    // 0.5
    println!("{}", lerp(21., 42., 0.3));
    // 27.3
    println!(
        "{}",
        lerp(Vector::from([2., 1.]), Vector::from([4., 2.]), 0.3)
    );
    // [2.6]
    // [1.3]
    println!(
        "{}",
        lerp(
            Matrix::from([[2., 1.], [3., 4.]]),
            Matrix::from([[20., 10.], [30., 40.]]),
            0.5
        )
    );
    // [[11., 5.5]
    // [16.5, 22.]]
}
