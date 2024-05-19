mod vector;

use vector::Vector;

#[cfg(test)]
mod vector_tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use nalgebra::DVector;
    use rand::prelude::*;

    const NB_TESTCASE_VECTORS: usize = 100;
    const THRESHOLD: f64 = 1e-10;

    fn test_norms(size: usize) {
        let mut rng = rand::thread_rng();
        let v = Vec::<f64>::from_iter((0..size).map(|_| rng.gen::<f64>()));

        let my_v = Vector::new(v.clone(), Some(size));
        let nalgebra_v = DVector::<f64>::from_vec(v);

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

    #[test]
    fn test_norms_vector3() {
        for _ in 0..NB_TESTCASE_VECTORS {
            test_norms(3);
        }
    }

    #[test]
    fn test_norms_vector5() {
        for _ in 0..NB_TESTCASE_VECTORS {
            test_norms(5);
        }
    }

    #[test]
    fn test_norms_random_size_vector() {
        for _ in 0..NB_TESTCASE_VECTORS {
            let mut rng = rand::thread_rng();
            test_norms(rng.gen_range(1..=10));
        }
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
