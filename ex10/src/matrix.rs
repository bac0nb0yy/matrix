use crate::field::*;
use crate::vector::Vector;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone)]
pub struct Matrix<K, const M: usize, const N: usize> {
    data: [[K; N]; M],
    rows: usize,
    cols: usize,
}

impl<K: Field + Display, const M: usize, const N: usize> Display for Matrix<K, M, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[")?;
        for (i, row) in self.data[..self.rows].iter().enumerate() {
            if i != 0 {
                write!(f, "\n ")?;
            }
            write!(f, "[")?;
            for (j, item) in row.iter().enumerate() {
                if j != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:.3}", item)?;
            }
            write!(f, "]")?;
        }
        writeln!(f, "]")
    }
}

impl<K: Field, const M: usize, const N: usize> Add<Matrix<K, M, N>> for Matrix<K, M, N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut data = [[K::zero(); N]; M];
        for i in 0..M {
            for j in 0..N {
                data[i][j] = self.data[i][j] + rhs.data[i][j];
            }
        }

        Matrix {
            data,
            rows: M,
            cols: N,
        }
    }
}

impl<K: Field, const M: usize, const N: usize> Add<Vector<K, N>> for Matrix<K, M, N> {
    type Output = Self;

    fn add(self, rhs: Vector<K, N>) -> Self::Output {
        let mut result = self;
        for i in 0..M {
            for j in 0..N {
                result.data[i][j] += rhs.data()[j];
            }
        }
        result
    }
}

impl<K: Field, const M: usize, const N: usize> Add<K> for Matrix<K, M, N> {
    type Output = Self;

    fn add(self, scalar: K) -> Self::Output {
        Matrix {
            data: self.data.map(|row| row.map(|val| val + scalar)),
            rows: M,
            cols: N,
        }
    }
}

impl<K: Field, const M: usize, const N: usize> AddAssign<Matrix<K, M, N>> for Matrix<K, M, N> {
    fn add_assign(&mut self, rhs: Matrix<K, M, N>) {
        self.data
            .iter_mut()
            .zip(&rhs.data)
            .for_each(|(row1, row2)| row1.iter_mut().zip(row2).for_each(|(a, &b)| *a += b));
    }
}

impl<K: Field, const M: usize, const N: usize> AddAssign<Vector<K, N>> for Matrix<K, M, N> {
    fn add_assign(&mut self, rhs: Vector<K, N>) {
        self.data.iter_mut().for_each(|row| {
            for (a, &b) in row.iter_mut().zip(rhs.data().iter()) {
                *a += b;
            }
        });
    }
}

impl<K: Field, const M: usize, const N: usize> AddAssign<K> for Matrix<K, M, N> {
    fn add_assign(&mut self, rhs: K) {
        self.data
            .iter_mut()
            .flat_map(|row| row.iter_mut())
            .for_each(|element| *element += rhs);
    }
}

impl<K: Field, const M: usize, const N: usize> Sub<Matrix<K, M, N>> for Matrix<K, M, N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut data = [[K::zero(); N]; M];
        for i in 0..M {
            for j in 0..N {
                data[i][j] = self.data[i][j] - rhs.data[i][j];
            }
        }

        Matrix {
            data,
            rows: M,
            cols: N,
        }
    }
}

impl<K: Field, const M: usize, const N: usize> Sub<Vector<K, N>> for Matrix<K, M, N> {
    type Output = Self;

    fn sub(self, rhs: Vector<K, N>) -> Self::Output {
        let mut result = self;
        for i in 0..M {
            for j in 0..N {
                result.data[i][j] -= rhs.data()[j];
            }
        }
        result
    }
}

impl<K: Field, const M: usize, const N: usize> Sub<K> for Matrix<K, M, N> {
    type Output = Self;

    fn sub(self, scalar: K) -> Self::Output {
        Matrix {
            data: self.data.map(|row| row.map(|val| val - scalar)),
            rows: M,
            cols: N,
        }
    }
}

impl<K: Field, const M: usize, const N: usize> SubAssign<Matrix<K, M, N>> for Matrix<K, M, N> {
    fn sub_assign(&mut self, rhs: Matrix<K, M, N>) {
        self.data
            .iter_mut()
            .zip(&rhs.data)
            .for_each(|(row1, row2)| row1.iter_mut().zip(row2).for_each(|(a, &b)| *a -= b));
    }
}

impl<K: Field, const M: usize, const N: usize> SubAssign<Vector<K, N>> for Matrix<K, M, N> {
    fn sub_assign(&mut self, rhs: Vector<K, N>) {
        self.data.iter_mut().for_each(|row| {
            for (a, &b) in row.iter_mut().zip(rhs.data().iter()) {
                *a -= b;
            }
        });
    }
}

impl<K: Field, const M: usize, const N: usize> SubAssign<K> for Matrix<K, M, N> {
    fn sub_assign(&mut self, rhs: K) {
        self.data
            .iter_mut()
            .flat_map(|row| row.iter_mut())
            .for_each(|element| *element -= rhs);
    }
}

impl<K: Field, const M: usize, const N: usize, const P: usize> Mul<Matrix<K, N, P>>
    for Matrix<K, M, N>
{
    type Output = Matrix<K, M, P>;

    fn mul(self, rhs: Matrix<K, N, P>) -> Self::Output {
        self.mul_mat(&rhs)
    }
}

impl<K: Field, const M: usize, const N: usize> Mul<Vector<K, N>> for Matrix<K, M, N> {
    type Output = Vector<K, M>;

    fn mul(self, rhs: Vector<K, N>) -> Self::Output {
        self.mul_vec(&rhs)
    }
}

impl<K: Field, const M: usize, const N: usize> Mul<K> for Matrix<K, M, N> {
    type Output = Self;

    fn mul(self, scalar: K) -> Self::Output {
        Matrix {
            data: self.data.map(|row| row.map(|val| val * scalar)),
            rows: M,
            cols: N,
        }
    }
}

impl<K: Field, const M: usize, const N: usize, const P: usize> MulAssign<Matrix<K, N, P>>
    for Matrix<K, M, N>
{
    fn mul_assign(&mut self, rhs: Matrix<K, N, P>) {
        for i in 0..M {
            for j in 0..P {
                for k in 0..N {
                    self.data[i][j] += self.data[i][k] * rhs.data[k][j];
                }
            }
        }
    }
}

impl<K: Field, const M: usize, const N: usize> MulAssign<K> for Matrix<K, M, N> {
    fn mul_assign(&mut self, rhs: K) {
        self.data
            .iter_mut()
            .flat_map(|row| row.iter_mut())
            .for_each(|element| *element *= rhs);
    }
}

impl<K: Field, const M: usize, const N: usize> Matrix<K, M, N> {
    pub fn new(data: [[K; N]; M]) -> Self {
        let rows: usize = data.len();
        let cols: usize = data[0].len();
        Matrix { data, rows, cols }
    }

    pub fn row_echelon(&self) -> Self {
        let mut result = self.clone();
        let mut pivot_row = 0;
        for pivot_col in 0..N {
            if pivot_row >= M {
                break;
            }

            let mut max_row = pivot_row;
            for row in (pivot_row + 1)..M {
                if result.data[row][pivot_col].abs() > result.data[max_row][pivot_col].abs() {
                    max_row = row;
                }
            }

            if result.data[max_row][pivot_col] == K::zero() {
                continue;
            }

            result.data.swap(pivot_row, max_row);
            let pivot = result.data[pivot_row][pivot_col];
            let mut row = [K::zero(); N];
            for col in 0..N {
                row[col] = result.data[pivot_row][col] / pivot;
            }

            result.data[pivot_row] = row;
            for row in (0..M).filter(|&r| r != pivot_row) {
                let factor = result.data[row][pivot_col];
                for col in pivot_col..N {
                    result.data[row][col] -= factor * result.data[pivot_row][col];
                }
            }

            pivot_row += 1;
        }
        result
    }

    pub fn mul_mat<const P: usize>(&self, rhs: &Matrix<K, N, P>) -> Matrix<K, M, P> {
        let mut data = [[K::zero(); P]; M];
        for i in 0..M {
            for j in 0..P {
                for k in 0..N {
                    data[i][j] += self.data[i][k] * rhs.data[k][j];
                }
            }
        }
        Matrix {
            data,
            rows: M,
            cols: P,
        }
    }

    pub fn mul_vec(&self, rhs: &Vector<K, N>) -> Vector<K, M> {
        let mut result = Vector::from([K::zero(); M]);
        for i in 0..M {
            for j in 0..N {
                result.data_mut()[i] += self.data[i][j] * rhs.data()[j];
            }
        }
        result
    }

    #[allow(dead_code)]
    pub fn data(&self) -> &[[K; N]; M] {
        &self.data
    }

    #[allow(dead_code)]
    pub fn data_mut(&mut self) -> &mut [[K; N]; M] {
        &mut self.data
    }

    #[allow(dead_code)]
    pub fn rows(&self) -> usize {
        self.rows
    }

    #[allow(dead_code)]
    pub fn cols(&self) -> usize {
        self.cols
    }
}

impl<K: Field, const M: usize, const N: usize> From<[[K; N]; M]> for Matrix<K, M, N> {
    fn from(array: [[K; N]; M]) -> Self {
        Matrix::new(array)
    }
}
