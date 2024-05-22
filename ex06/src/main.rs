mod field;
mod vector;

use field::*;
use vector::Vector;

fn cross_product<K: Field>(u: &Vector<K, 3>, v: &Vector<K, 3>) -> Vector<K, 3> {
    Vector::new([
        u.data()[1] * v.data()[2] - u.data()[2] * v.data()[1],
        u.data()[2] * v.data()[0] - u.data()[0] * v.data()[2],
        u.data()[0] * v.data()[1] - u.data()[1] * v.data()[0],
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use nalgebra::SVector;
    use rand::prelude::*;

    const NB_TESTCASE_VECTORS: usize = 100;
    const THRESHOLD: f64 = 1e-10;

    fn test_cross_product() {
        for _ in 0..NB_TESTCASE_VECTORS {
            let mut rng = rand::thread_rng();
            let v1: [f64; 3] = (0..3)
                .map(|_| rng.gen())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let v2: [f64; 3] = (0..3)
                .map(|_| rng.gen())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            let my_v1 = Vector::new(v1);
            let my_v2 = Vector::new(v2);

            let nalgebra_v1 = SVector::<f64, 3>::from_vec(v1.to_vec());
            let nalgebra_v2 = SVector::<f64, 3>::from_vec(v2.to_vec());

            let my_cross_product = cross_product(&my_v1, &my_v2);
            let nalgebra_cross_product = nalgebra_v1.cross(&nalgebra_v2);

            for (i, &value) in my_cross_product.data().iter().enumerate() {
                assert_abs_diff_eq!(value, nalgebra_cross_product[i], epsilon = THRESHOLD);
            }
        }
    }

    #[test]
    fn test_cross_product_vector() {
        test_cross_product();
    }
}

fn main() {
    let u = Vector::from([0., 0., 1.]);
    let v = Vector::from([1., 0., 0.]);
    println!("{}", cross_product(&u, &v));
    // [0.]
    // [1.]
    // [0.]
    let u = Vector::from([1., 2., 3.]);
    let v = Vector::from([4., 5., 6.]);
    println!("{}", cross_product(&u, &v));
    // [-3.]
    // [6.]
    // [-3.]
    let u = Vector::from([4., 2., -3.]);
    let v = Vector::from([-2., -5., 16.]);
    println!("{}", cross_product(&u, &v));
    // [17.]
    // [-58.]
    // [-16.]
}
