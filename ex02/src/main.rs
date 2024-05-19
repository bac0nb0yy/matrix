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
    use nalgebra::{DVector, DefaultAllocator, Scalar};
    use rand::prelude::*;

    const NB_TESTCASE_VECTORS: usize = 100;
    const THRESHOLD: f64 = 1e-10;

    fn test_lerp<N: Scalar + Copy + nalgebra::DimName>(size: usize)
    where
        DefaultAllocator: nalgebra::allocator::Allocator<f64, N>,
    {
        let mut rng = rand::thread_rng();
        let v1 = Vec::<f64>::from_iter((0..size).map(|_| rng.gen::<f64>()));
        let v2 = Vec::<f64>::from_iter((0..size).map(|_| rng.gen::<f64>()));

        let my_v1 = Vector::new(Vec::from(v1.clone()), Some(size));
        let my_v2 = Vector::new(Vec::from(v2.clone()), Some(size));
        let nalgebra_v1 = DVector::<f64>::from_vec(v1);
        let nalgebra_v2 = DVector::<f64>::from_vec(v2);

        let t: f64 = rng.gen();

        let my_result = lerp(my_v1, my_v2, t);
        let nalgebra_result = nalgebra_v1.lerp(&nalgebra_v2, t);

        for (i, &value) in my_result.get_data().iter().enumerate() {
            assert!(
                (value - nalgebra_result[i]).abs() < THRESHOLD,
                "Mismatch at index {}: {} != {}",
                i,
                value,
                nalgebra_result[i]
            );
        }
    }

    #[test]
    fn test_lerp_vector3() {
        for _ in 0..NB_TESTCASE_VECTORS {
            test_lerp::<nalgebra::U3>(3);
        }
    }

    #[test]
    fn test_lerp_vector5() {
        for _ in 0..NB_TESTCASE_VECTORS {
            test_lerp::<nalgebra::U5>(5);
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
