mod field;
mod matrix;
mod vector;

use matrix::Matrix;

fn main() {
    let u = Matrix::from([[2., 5., 3.], [1., -2., -1.], [1., 3., 4.]]);
    println!("{}", u.determinant());
    // -20.0
    let u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
    println!("{}", u.determinant());
    // 8.0
    let u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
    println!("{}", u.determinant());
    // -174.0
    let u = Matrix::from([
        [8., 5., -2., 4.],
        [4., 2.5, 20., 4.],
        [8., 5., 1., 4.],
        [28., -4., 17., 1.],
    ]);
    println!("{}", u.determinant());
    // 1032
}
