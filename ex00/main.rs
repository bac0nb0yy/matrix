mod matrix;
mod vector;

use matrix::Matrix;
use vector::Vector;

fn main() {
    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    u.add(&v);
    println!("{}", u);
    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    u.sub(&v);
    println!("{}", u);
    let mut u = Vector::from([2., 3.]);
    u.scl(2.);
    println!("{}", u);
    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    let v = Matrix::from([[7., 4.], [-2., 2.]]);
    u.add(&v);
    println!("{}", u);
    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    let v = Matrix::from([[7., 4.], [-2., 2.]]);
    u.sub(&v);
    println!("{}", u);
    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    u.scl(2.);
    println!("{}", u);
}
