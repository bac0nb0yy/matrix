use crate::field::*;

use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone)]
pub struct Vector<K, const N: usize> {
    data: [K; N],
    dim: usize,
}

impl<K: Field + Display, const N: usize> Display for Vector<K, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[")?;
        for (index, item) in self.data.iter().enumerate() {
            if index != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:.3}", item)?;
        }
        writeln!(f, "]")?;
        Ok(())
    }
}

impl<K: Field, const N: usize> Add<Vector<K, N>> for Vector<K, N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut data = [K::zero(); N];
        for i in 0..N {
            data[i] = self.data[i] + rhs.data[i];
        }

        Vector { data, dim: N }
    }
}

impl<K: Field, const N: usize> Add<K> for Vector<K, N> {
    type Output = Self;

    fn add(self, scalar: K) -> Self::Output {
        Vector {
            data: self.data.map(|a| a + scalar),
            dim: N,
        }
    }
}

impl<K: Field, const N: usize> AddAssign<Vector<K, N>> for Vector<K, N> {
    fn add_assign(&mut self, rhs: Vector<K, N>) {
        self.data
            .iter_mut()
            .zip(&rhs.data)
            .for_each(|(a, &b)| *a += b);
    }
}

impl<K: Field, const N: usize> AddAssign<K> for Vector<K, N> {
    fn add_assign(&mut self, rhs: K) {
        self.data.iter_mut().for_each(|a| *a += rhs);
    }
}

impl<K: Field, const N: usize> Sub<Vector<K, N>> for Vector<K, N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut data = [K::zero(); N];
        for i in 0..N {
            data[i] = self.data[i] - rhs.data[i];
        }

        Vector { data, dim: N }
    }
}

impl<K: Field, const N: usize> Sub<K> for Vector<K, N> {
    type Output = Self;

    fn sub(self, scalar: K) -> Self::Output {
        Vector {
            data: self.data.map(|a| a - scalar),
            dim: N,
        }
    }
}

impl<K: Field, const N: usize> SubAssign<Vector<K, N>> for Vector<K, N> {
    fn sub_assign(&mut self, rhs: Vector<K, N>) {
        self.data
            .iter_mut()
            .zip(&rhs.data)
            .for_each(|(a, &b)| *a -= b);
    }
}

impl<K: Field, const N: usize> SubAssign<K> for Vector<K, N> {
    fn sub_assign(&mut self, scalar: K) {
        self.data.iter_mut().for_each(|a| *a -= scalar);
    }
}

impl<K: Field, const N: usize> Mul<Vector<K, N>> for Vector<K, N> {
    type Output = K;

    fn mul(self, rhs: Vector<K, N>) -> K {
        self.dot(&rhs)
    }
}

impl<K: Field, const N: usize> Mul<K> for Vector<K, N> {
    type Output = Self;

    fn mul(self, scalar: K) -> Self::Output {
        Vector {
            data: self.data.map(|a| a * scalar),
            dim: N,
        }
    }
}

impl<K: Field, const N: usize> MulAssign<Vector<K, N>> for Vector<K, N> {
    fn mul_assign(&mut self, rhs: Vector<K, N>) {
        self.data
            .iter_mut()
            .zip(&rhs.data)
            .for_each(|(a, &b)| *a *= b);
    }
}

impl<K: Field, const N: usize> MulAssign<K> for Vector<K, N> {
    fn mul_assign(&mut self, scl: K) {
        self.data.iter_mut().for_each(|a| *a *= scl);
    }
}

impl<K: Field, const N: usize> Div<Vector<K, N>> for Vector<K, N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let mut data = [K::zero(); N];
        for i in 0..N {
            data[i] = self.data[i] / rhs.data[i];
        }

        Vector { data, dim: N }
    }
}

impl<K: Field, const N: usize> Div<K> for Vector<K, N> {
    type Output = Self;

    fn div(self, scalar: K) -> Self::Output {
        Vector {
            data: self.data.map(|a| a / scalar),
            dim: N,
        }
    }
}

impl<K: Field, const N: usize> DivAssign<Vector<K, N>> for Vector<K, N> {
    fn div_assign(&mut self, rhs: Vector<K, N>) {
        self.data
            .iter_mut()
            .zip(&rhs.data)
            .for_each(|(a, &b)| *a /= b);
    }
}

impl<K: Field, const N: usize> DivAssign<K> for Vector<K, N> {
    fn div_assign(&mut self, scalar: K) {
        self.data.iter_mut().for_each(|a| *a /= scalar);
    }
}

impl<K: Field, const N: usize> Neg for Vector<K, N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector {
            data: self.data.map(|x| -x),
            dim: self.dim,
        }
    }
}

impl<K: Field, const N: usize> Vector<K, N> {
    pub fn dot(&self, v: &Vector<K, N>) -> K {
        self.data
            .iter()
            .zip(&v.data)
            .fold(K::zero(), |acc, (&x, &y)| acc + x * y)
    }

    pub fn linear_combination(u: &[Vector<K, N>], coefs: &[K]) -> Vector<K, N> {
        assert_eq!(
            N,
            coefs.len(),
            "Number of vectors and coefficients must match"
        );

        let mut result = [K::zero(); N];
        for (vector, coef) in u.iter().zip(coefs.iter()) {
            for (a, b) in result.iter_mut().zip(&vector.data) {
                *a = *a + *coef * *b;
            }
        }

        Vector {
            data: result,
            dim: N,
        }
    }

    pub fn new(data: [K; N]) -> Self {
        Vector { data, dim: N }
    }

    #[allow(dead_code)]
    pub fn data(&self) -> &[K; N] {
        &self.data
    }

    #[allow(dead_code)]
    pub fn data_mut(&mut self) -> &mut [K; N] {
        &mut self.data
    }
}

impl<K: Field, const N: usize> From<[K; N]> for Vector<K, N> {
    fn from(array: [K; N]) -> Self {
        Vector::new(array)
    }
}
