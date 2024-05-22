mod field;
mod matrix;
mod vector;

use field::*;
use matrix::Matrix;
use std::fmt::Display;

fn print_inverse<K: Field + Display, const N: usize>(matrix: Matrix<K, N, N>) {
    match matrix.clone().inverse() {
        Ok(inverse) => println!("{}", inverse),
        Err(_) => println!("The matrix is singular and cannot be inverted"),
    }
}

fn main() {
    print_inverse(Matrix::from([[2., 4.], [1., 2.]]));
    // error
    print_inverse(Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]));
    // [1.0, 0.0, 0.0]
    // [0.0, 1.0, 0.0]
    // [0.0, 0.0, 1.0]
    print_inverse(Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]));
    // [0.5, 0.0, 0.0]
    // [0.0, 0.5, 0.0]
    // [0.0, 0.0, 0.5]
    print_inverse(Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]));
    // [0.649425287, 0.097701149, -0.655172414]
    // [-0.781609195, -0.126436782, 0.965517241]
    // [0.143678161, 0.074712644, -0.206896552]
}
