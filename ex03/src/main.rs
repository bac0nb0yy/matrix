mod vector;

use vector::Vector;

#[cfg(test)]
mod dot_product_tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use nalgebra::{allocator::Allocator, DVector, DefaultAllocator, DimName, Scalar, U3, U5};
    use rand::prelude::*;

    const NB_TESTCASE_VECTORS: usize = 100;
    const THRESHOLD: f64 = 1e-10;

    fn test_dot<N: Scalar + DimName + Copy>(size: usize)
    where
        DefaultAllocator: Allocator<f64, N>,
    {
        let mut rng = rand::thread_rng();
        let v1 = Vec::<f64>::from_iter((0..size).map(|_| rng.gen::<f64>()));
        let v2 = Vec::<f64>::from_iter((0..size).map(|_| rng.gen::<f64>()));

        let my_v1 = Vector::new(v1.clone(), Some(size));
        let my_v2 = Vector::new(v2.clone(), Some(size));
        let nalgebra_v1 = DVector::<f64>::from_vec(v1);
        let nalgebra_v2 = DVector::<f64>::from_vec(v2);

        assert_abs_diff_eq!(
            my_v1.dot(&my_v2),
            nalgebra_v1.dot(&nalgebra_v2),
            epsilon = THRESHOLD
        );
    }

    #[test]
    fn test_dot_vector3() {
        for _ in 0..NB_TESTCASE_VECTORS {
            test_dot::<U3>(3);
        }
    }

    #[test]
    fn test_dot_vector5() {
        for _ in 0..NB_TESTCASE_VECTORS {
            test_dot::<U5>(5);
        }
    }
}

fn main() {
    let u = Vector::from([0., 0.]);
    let v = Vector::from([1., 1.]);
    println!("{}", u.dot(&v));
    // 0.0
    let u = Vector::from([1., 1.]);
    let v = Vector::from([1., 1.]);
    println!("{}", u.dot(&v));
    // 2.0
    let u = Vector::from([-1., 6.]);
    let v = Vector::from([3., 2.]);
    println!("{}", u.dot(&v));
    // 9.0
}
