mod field;
mod vector;

use vector::Vector;

#[cfg(test)]
mod vector_tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use nalgebra::SVector;
    use rand::prelude::*;

    const NB_TESTCASE_VECTORS: usize = 100;
    const THRESHOLD: f64 = 1e-10;

    fn test_norms<const N: usize>() {
        for _ in 0..NB_TESTCASE_VECTORS {
            let mut rng = rand::thread_rng();
            let v: [f64; N] = (0..N)
                .map(|_| rng.gen())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            let my_v = Vector::new(v.clone());
            let nalgebra_v = SVector::<f64, N>::from_vec(v.to_vec());

            assert_abs_diff_eq!(
                my_v.norm_1(),
                nalgebra_v.iter().fold(0.0, |acc, &x| acc + x.abs()),
                epsilon = THRESHOLD
            );

            assert_abs_diff_eq!(
                my_v.norm(),
                nalgebra_v.iter().fold(0.0, |acc, &x| acc + x * x).sqrt(),
                epsilon = THRESHOLD
            );

            assert_abs_diff_eq!(
                my_v.norm_inf(),
                nalgebra_v
                    .iter()
                    .fold(f64::NEG_INFINITY, |max, &x| x.abs().max(max)),
                epsilon = THRESHOLD
            );
        }
    }

    #[test]
    fn test_norms_vector3() {
        test_norms::<3>();
    }

    #[test]
    fn test_norms_vector5() {
        test_norms::<5>();
    }

    #[test]
    fn test_norms_vector42() {
        test_norms::<42>();
    }

    #[test]
    fn test_norms_vector69() {
        test_norms::<69>();
    }
}

fn main() {
    let u = Vector::from([0., 0., 0.]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 0.0, 0.0, 0.0
    let u = Vector::from([1., 2., 3.]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 6.0, 3.74165738, 3.0
    let u = Vector::from([-1., -2.]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 3.0, 2.236067977, 2.0
}
