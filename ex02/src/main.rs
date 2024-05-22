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

    #[test]
    fn test_lerp_vector3() {
        for _ in 0..NB_TESTCASE_VECTORS {
            test_lerp::<3>();
        }
    }

    #[test]
    fn test_lerp_vector5() {
        for _ in 0..NB_TESTCASE_VECTORS {
            test_lerp::<5>();
        }
    }

    #[test]
    fn test_lerp_vector42() {
        for _ in 0..NB_TESTCASE_VECTORS {
            test_lerp::<42>();
        }
    }

    #[test]
    fn test_lerp_vector69() {
        for _ in 0..NB_TESTCASE_VECTORS {
            test_lerp::<69>();
        }
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
