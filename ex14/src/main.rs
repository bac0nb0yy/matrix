mod field;
mod matrix;
mod vector;

use matrix::Matrix;

use std::f32::consts::PI;

fn projection(fov: f32, ratio: f32, near: f32, far: f32) -> Matrix<f32, 4, 4> {
    let scale = 1.0 / (fov / 2.0).tan();

    Matrix::from([
        [scale / ratio, 0.0, 0.0, 0.0],
        [0.0, scale, 0.0, 0.0],
        [0.0, 0.0, (far + near) / (near - far), -1.],
        [0.0, 0.0, (2.0 * far * near) / (near - far), 0.0],
    ])
}

fn print_formatted(matrice: &Matrix<f32, 4, 4>) {
    for i in 0..4 {
        for j in 0..4 {
            if j != 3 {
                print!("{}, ", matrice[i][j]);
            } else {
                print!("{}", matrice[i][j]);
            }
        }
        if i != 3 {
            println!();
        }
    }
}

fn main() {
    let fov = 60.0 * PI / 180.0;
    let ratio = 1080.0 / 1080.0;
    let near = 1.;
    let far = 100.0;

    let projection_matrix = projection(fov, ratio, near, far);
    print_formatted(&projection_matrix);
}
