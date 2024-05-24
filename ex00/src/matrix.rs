use crate::field::*;
use crate::vector::Vector;

use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix<K, const M: usize, const N: usize> {
    data: [[K; N]; M],
}

impl<K: Field + Display, const M: usize, const N: usize> Display for Matrix<K, M, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[")?;
        for (i, row) in self.iter().enumerate() {
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

impl<K: Field, const M: usize, const N: usize> Deref for Matrix<K, M, N> {
    type Target = [[K; N]; M];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<K: Field, const M: usize, const N: usize> DerefMut for Matrix<K, M, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<K: Field, const M: usize, const N: usize> Add<Matrix<K, M, N>> for Matrix<K, M, N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut data = [[K::zero(); N]; M];
        for i in 0..M {
            for j in 0..N {
                data[i][j] = self[i][j] + rhs[i][j];
            }
        }

        Matrix { data }
    }
}

impl<K: Field, const M: usize, const N: usize> Add<Vector<K, N>> for Matrix<K, M, N> {
    type Output = Self;

    fn add(self, rhs: Vector<K, N>) -> Self::Output {
        let mut result = self;
        for i in 0..M {
            for j in 0..N {
                result[i][j] += rhs[j];
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
        }
    }
}

impl<K: Field, const M: usize, const N: usize> AddAssign<Matrix<K, M, N>> for Matrix<K, M, N> {
    fn add_assign(&mut self, rhs: Matrix<K, M, N>) {
        self.iter_mut()
            .zip(&rhs.data)
            .for_each(|(row1, row2)| row1.iter_mut().zip(row2).for_each(|(a, &b)| *a += b));
    }
}

impl<K: Field, const M: usize, const N: usize> AddAssign<Vector<K, N>> for Matrix<K, M, N> {
    fn add_assign(&mut self, rhs: Vector<K, N>) {
        self.iter_mut().for_each(|row| {
            for (a, &b) in row.iter_mut().zip(rhs.iter()) {
                *a += b;
            }
        });
    }
}

impl<K: Field, const M: usize, const N: usize> AddAssign<K> for Matrix<K, M, N> {
    fn add_assign(&mut self, rhs: K) {
        self.iter_mut()
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
                data[i][j] = self[i][j] - rhs[i][j];
            }
        }

        Matrix { data }
    }
}

impl<K: Field, const M: usize, const N: usize> Sub<Vector<K, N>> for Matrix<K, M, N> {
    type Output = Self;

    fn sub(self, rhs: Vector<K, N>) -> Self::Output {
        let mut result = self;
        for i in 0..M {
            for j in 0..N {
                result[i][j] -= rhs[j];
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
        }
    }
}

impl<K: Field, const M: usize, const N: usize> SubAssign<Matrix<K, M, N>> for Matrix<K, M, N> {
    fn sub_assign(&mut self, rhs: Matrix<K, M, N>) {
        self.iter_mut()
            .zip(&rhs.data)
            .for_each(|(row1, row2)| row1.iter_mut().zip(row2).for_each(|(a, &b)| *a -= b));
    }
}

impl<K: Field, const M: usize, const N: usize> SubAssign<Vector<K, N>> for Matrix<K, M, N> {
    fn sub_assign(&mut self, rhs: Vector<K, N>) {
        self.iter_mut().for_each(|row| {
            for (a, &b) in row.iter_mut().zip(rhs.iter()) {
                *a -= b;
            }
        });
    }
}

impl<K: Field, const M: usize, const N: usize> SubAssign<K> for Matrix<K, M, N> {
    fn sub_assign(&mut self, rhs: K) {
        self.iter_mut()
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
                    self[i][j] = self[i][j] + self[i][k] * rhs[k][j];
                }
            }
        }
    }
}

impl<K: Field, const M: usize, const N: usize> MulAssign<K> for Matrix<K, M, N> {
    fn mul_assign(&mut self, rhs: K) {
        self.iter_mut()
            .flat_map(|row| row.iter_mut())
            .for_each(|element| *element *= rhs);
    }
}

impl<K: Field, const M: usize, const N: usize> Div<K> for Matrix<K, M, N> {
    type Output = Self;

    fn div(self, scalar: K) -> Self::Output {
        Matrix {
            data: self.data.map(|row| row.map(|val| val / scalar)),
        }
    }
}

impl<K: Field, const M: usize, const N: usize> DivAssign<K> for Matrix<K, M, N> {
    fn div_assign(&mut self, rhs: K) {
        self.iter_mut()
            .flat_map(|row| row.iter_mut())
            .for_each(|element| *element /= rhs);
    }
}

impl<K: Field, const M: usize, const N: usize> Matrix<K, M, N> {
    pub fn new(data: [[K; N]; M]) -> Self {
        Matrix { data }
    }

    fn operate<F: Fn(K, K) -> K>(&mut self, v: &Matrix<K, M, N>, op: F) {
        self.iter_mut().zip(&v.data).for_each(|(a_row, b_row)| {
            a_row
                .iter_mut()
                .zip(b_row)
                .for_each(|(a, b)| *a = op(*a, *b));
        });
    }

    pub fn add(&mut self, v: &Matrix<K, M, N>) {
        self.operate(v, |a, b| a + b);
    }

    pub fn sub(&mut self, v: &Matrix<K, M, N>) {
        self.operate(v, |a, b| a - b);
    }

    pub fn scl(&mut self, a: K) {
        self.iter_mut()
            .for_each(|row| row.iter_mut().for_each(|v| *v = *v * a));
    }

    pub fn inv_scl(&mut self, a: K) {
        self.iter_mut()
            .for_each(|row| row.iter_mut().for_each(|v| *v = *v / a));
    }

    pub fn mul_mat<const P: usize>(&self, rhs: &Matrix<K, N, P>) -> Matrix<K, M, P> {
        let mut data = [[K::zero(); P]; M];
        for i in 0..M {
            for j in 0..P {
                for k in 0..N {
                    data[i][j] += self[i][k] * rhs[k][j];
                }
            }
        }
        Matrix { data }
    }

    pub fn mul_vec(&self, rhs: &Vector<K, N>) -> Vector<K, M> {
        let mut result = Vector::from([K::zero(); M]);
        for i in 0..M {
            for j in 0..N {
                result[i] += self[i][j] * rhs[j];
            }
        }
        result
    }
}

impl<K: Field, const M: usize, const N: usize> From<[[K; N]; M]> for Matrix<K, M, N> {
    fn from(array: [[K; N]; M]) -> Self {
        Matrix::new(array)
    }
}
