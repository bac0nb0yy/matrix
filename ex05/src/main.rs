mod field;
mod vector;

use field::*;
use vector::Vector;

fn angle_cos<K: Field, const N: usize>(u: &Vector<K, N>, v: &Vector<K, N>) -> f32 {
    let norm_u = u.norm();
    let norm_v = v.norm();

    if norm_u == K::zero().into() || norm_v == K::zero().into() {
        panic!("One or both vectors are zero");
    }

    (u.dot(v).into() / (u.norm() * v.norm())) as f32
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use nalgebra::SVector;
    use rand::prelude::*;

    const NB_TESTCASE_VECTORS: usize = 100;
    const THRESHOLD: f32 = 1e-10;

    fn test_cos<const N: usize>() {
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

            let my_v1 = Vector::new(v1);
            let my_v2 = Vector::new(v2);

            let nalgebra_v1 = SVector::<f64, N>::from_vec(v1.to_vec());
            let nalgebra_v2 = SVector::<f64, N>::from_vec(v2.to_vec());

            let cos_angle = angle_cos(&my_v1, &my_v2);
            let expected_cos_angle = nalgebra_v1.normalize().dot(&nalgebra_v2.normalize());

            assert_abs_diff_eq!(cos_angle, expected_cos_angle as f32, epsilon = THRESHOLD);
        }
    }

    #[test]
    fn test_cos_vector3() {
        test_cos::<3>();
    }

    #[test]
    fn test_cos_vector5() {
        test_cos::<5>();
    }

    #[test]
    fn test_cos_vector42() {
        test_cos::<42>();
    }

    #[test]
    fn test_cos_vector69() {
        test_cos::<69>();
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
