mod field;
mod vector;

use vector::Vector;

#[cfg(test)]
mod dot_product_tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use nalgebra::SVector;
    use rand::prelude::*;

    const NB_TESTCASE_VECTORS: usize = 100;
    const THRESHOLD: f64 = 1e-10;

    fn test_dot<const N: usize>() {
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

            assert_abs_diff_eq!(
                my_v1.dot(&my_v2),
                nalgebra_v1.dot(&nalgebra_v2),
                epsilon = THRESHOLD
            );
        }
    }

    #[test]
    fn test_dot_vector3() {
        test_dot::<3>();
    }

    #[test]
    fn test_dot_vector5() {
        test_dot::<5>();
    }

    #[test]
    fn test_dot_vector42() {
        test_dot::<42>();
    }

    #[test]
    fn test_dot_vector69() {
        test_dot::<69>();
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
