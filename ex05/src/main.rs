mod field;
mod vector;

use field::*;
use vector::Vector;

fn angle_cos<K: Field>(u: &Vector<K>, v: &Vector<K>) -> f32 {
    (u.dot(v).into() / (u.norm() * v.norm())) as f32
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use nalgebra::DVector;
    use rand::prelude::*;

    const NB_TESTCASE_VECTORS: usize = 100;
    const THRESHOLD: f32 = 1e-10;

    fn test_cos(size: usize) {
        for _ in 0..NB_TESTCASE_VECTORS {
            let mut rng = rand::thread_rng();
            let v1 = Vec::<f64>::from_iter((0..size).map(|_| rng.gen::<f64>()));
            let v2 = Vec::<f64>::from_iter((0..size).map(|_| rng.gen::<f64>()));

            let my_v1 = Vector::new(v1);
            let my_v2 = Vector::new(v2);

            let nalgebra_v1 = DVector::<f64>::from_vec(my_v1.data().clone());
            let nalgebra_v2 = DVector::<f64>::from_vec(my_v2.data().clone());

            let cos_angle = angle_cos(&my_v1, &my_v2);
            let expected_cos_angle = nalgebra_v1.normalize().dot(&nalgebra_v2.normalize());

            assert_abs_diff_eq!(cos_angle, expected_cos_angle as f32, epsilon = THRESHOLD);
        }
    }

    #[test]
    fn test_cos_vector3() {
        for _ in 0..NB_TESTCASE_VECTORS {
            test_cos(3);
        }
    }

    #[test]
    fn test_cos_vector5() {
        for _ in 0..NB_TESTCASE_VECTORS {
            test_cos(5);
        }
    }

    #[test]
    fn test_cos_random_size_vector() {
        for _ in 0..NB_TESTCASE_VECTORS {
            let mut rng = rand::thread_rng();
            test_cos(rng.gen_range(1..=10));
        }
    }
}

fn main() {
    let u = Vector::from([1., 0.]);
    let v = Vector::from([1., 0.]);
    println!("{}", angle_cos(&u, &v));
    // 1.0
    let u = Vector::from([1., 0.]);
    let v = Vector::from([0., 1.]);
    println!("{}", angle_cos(&u, &v));
    // 0.0
    let u = Vector::from([-1., 1.]);
    let v = Vector::from([1., -1.]);
    println!("{}", angle_cos(&u, &v));
    // -1.0
    let u = Vector::from([2., 1.]);
    let v = Vector::from([4., 2.]);
    println!("{}", angle_cos(&u, &v));
    // 1.0
    let u = Vector::from([1., 2., 3.]);
    let v = Vector::from([4., 5., 6.]);
    println!("{}", angle_cos(&u, &v));
    // 0.974631846
}
