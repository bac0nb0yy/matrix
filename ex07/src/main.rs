mod field;
mod matrix;
mod vector;

use matrix::Matrix;
use vector::Vector;

fn main() {
    let matrix: Matrix<i32, 2, 2> = Matrix::new([[1, 2], [3, 4]]);
    println!("{}", matrix + Vector::from([1, 2]));
}
