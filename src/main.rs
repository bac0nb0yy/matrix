use std::f32::consts::PI;

#[derive(Debug)]
pub struct Matrix<T, const M: usize, const N: usize> {
    data: [[T; N]; M],
}

impl Matrix<f32, 4, 4> {
    fn projection(fov: f32, ratio: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fov / 2.0).tan();

        Self {
            data: [
                [f / ratio, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [
                    0.0,
                    0.0,
                    (far + near) / (near - far),
                    (2.0 * far * near) / (near - far),
                ],
                [0.0, 0.0, -1.0, 0.0],
            ],
        }
    }
}

fn main() {
    let fov = 90.0 * PI / 180.0;
    let ratio = 800.0 / 600.0;
    let near = 0.1;
    let far = 100.0;

    let projection_matrix = Matrix::projection(fov, ratio, near, far);

    println!("{:?}", projection_matrix);
}
