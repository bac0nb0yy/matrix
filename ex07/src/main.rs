mod field;
mod vector;

use field::*;
use vector::Vector;

fn cross_product<K: Field>(u: &Vector<K>, v: &Vector<K>) -> Vector<K> {
    if u.size() != 3 || v.size() != 3 {
        panic!("Cross product is only defined for 3-dimensional vectors.")
    }

    Vector::new(vec![
        u.data()[1] * v.data()[2] - u.data()[2] * v.data()[1],
        u.data()[2] * v.data()[0] - u.data()[0] * v.data()[2],
        u.data()[0] * v.data()[1] - u.data()[1] * v.data()[0],
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use nalgebra::DVector;
    use rand::prelude::*;

    const NB_TESTCASE_VECTORS: usize = 100;
    const THRESHOLD: f64 = 1e-10;

    fn test_cross_product(size: usize) {
        for _ in 0..NB_TESTCASE_VECTORS {
            let mut rng = rand::thread_rng();
            let v1 = Vec::<f64>::from_iter((0..size).map(|_| rng.gen::<f64>()));
            let v2 = Vec::<f64>::from_iter((0..size).map(|_| rng.gen::<f64>()));

            let my_v1 = Vector::new(v1);
            let my_v2 = Vector::new(v2);

            let nalgebra_v1 = DVector::<f64>::from_vec(my_v1.data().clone());
            let nalgebra_v2 = DVector::<f64>::from_vec(my_v2.data().clone());

            let my_cross_product = cross_product(&my_v1, &my_v2);
            let nalgebra_cross_product = nalgebra_v1.cross(&nalgebra_v2);

            for (i, &value) in my_cross_product.data().iter().enumerate() {
                assert_abs_diff_eq!(value, nalgebra_cross_product[i], epsilon = THRESHOLD);
            }
        }
    }

    #[test]
    fn test_cross_product_vector3() {
        for _ in 0..NB_TESTCASE_VECTORS {
            test_cross_product(3);
        }
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
