mod field;
mod matrix;
mod vector;

use matrix::Matrix;

fn main() {
    let matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);

    let transposed = matrix.transpose();

    println!("{}", transposed);
    // [[1, 4], [2, 5], [3, 6]]
}
