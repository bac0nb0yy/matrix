use crate::field::*;

use std::fmt::{Display, Formatter, Result};
use std::ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<K, const N: usize> {
    data: [K; N],
}

impl<K: Field + Display, const N: usize> Display for Vector<K, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[")?;
        for (index, item) in self.iter().enumerate() {
            if index != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:.3}", item)?;
        }
        writeln!(f, "]")?;
        Ok(())
    }
}

impl<K: Field, const N: usize> Deref for Vector<K, N> {
    type Target = [K];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<K: Field, const N: usize> DerefMut for Vector<K, N> {
    fn deref_mut(&mut self) -> &mut [K] {
        &mut self.data
    }
}

impl<K, const N: usize> IntoIterator for Vector<K, N> {
    type Item = K;
    type IntoIter = std::array::IntoIter<K, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<K: Field, const N: usize> Add<Vector<K, N>> for Vector<K, N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut data = self;
        data.operate(&rhs, |a, b| a + b);

        data
    }
}

impl<K: Field, const N: usize> Add<K> for Vector<K, N> {
    type Output = Self;

    fn add(self, scalar: K) -> Self::Output {
        Vector {
            data: self.data.map(|a| a + scalar),
        }
    }
}

impl<K: Field, const N: usize> AddAssign<Vector<K, N>> for Vector<K, N> {
    fn add_assign(&mut self, rhs: Vector<K, N>) {
        self.operate(&rhs, |a, b| a + b);
    }
}

impl<K: Field, const N: usize> AddAssign<K> for Vector<K, N> {
    fn add_assign(&mut self, rhs: K) {
        self.iter_mut().for_each(|a| *a += rhs);
    }
}

impl<K: Field, const N: usize> Sub<Vector<K, N>> for Vector<K, N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut data = self;
        data.operate(&rhs, |a, b| a - b);

        data
    }
}

impl<K: Field, const N: usize> Sub<K> for Vector<K, N> {
    type Output = Self;

    fn sub(self, scalar: K) -> Self::Output {
        Vector {
            data: self.data.map(|a| a - scalar),
        }
    }
}

impl<K: Field, const N: usize> SubAssign<Vector<K, N>> for Vector<K, N> {
    fn sub_assign(&mut self, rhs: Vector<K, N>) {
        self.operate(&rhs, |a, b| a - b);
    }
}

impl<K: Field, const N: usize> SubAssign<K> for Vector<K, N> {
    fn sub_assign(&mut self, scalar: K) {
        self.iter_mut().for_each(|a| *a -= scalar);
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
        }
    }
}

impl<K: Field, const N: usize> MulAssign<K> for Vector<K, N> {
    fn mul_assign(&mut self, scalar: K) {
        self.scl(scalar);
    }
}

impl<K: Field, const N: usize> Div<K> for Vector<K, N> {
    type Output = Self;

    fn div(self, scalar: K) -> Self::Output {
        if scalar == K::zero() {
            panic!("Division by zero error")
        }

        Vector {
            data: self.data.map(|a| a / scalar),
        }
    }
}

impl<K: Field, const N: usize> DivAssign<K> for Vector<K, N> {
    fn div_assign(&mut self, scalar: K) {
        self.inv_scl(scalar);
    }
}

impl<K: Field, const N: usize> Neg for Vector<K, N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector {
            data: self.data.map(|x| -x),
        }
    }
}

impl<K: Field, const N: usize> Vector<K, N> {
    fn operate<F: Fn(K, K) -> K>(&mut self, v: &Vector<K, N>, op: F) {
        self.iter_mut()
            .zip(&v.data)
            .for_each(|(a, b)| *a = op(*a, *b));
    }

    pub fn new(data: [K; N]) -> Self {
        Vector { data }
    }

    pub fn add(&mut self, v: &Vector<K, N>) {
        self.operate(v, |a, b| a + b);
    }

    pub fn sub(&mut self, v: &Vector<K, N>) {
        self.operate(v, |a, b| a - b);
    }

    pub fn scl(&mut self, a: K) {
        self.iter_mut().for_each(|v| *v = *v * a);
    }

    pub fn inv_scl(&mut self, a: K) {
        self.iter_mut().for_each(|v| *v = *v / a);
    }

    pub fn dot(&self, v: &Vector<K, N>) -> K {
        self.iter()
            .zip(&v.data)
            .fold(K::zero(), |acc, (&x, &y)| acc + x * y)
    }
}

impl<K: Field, const N: usize> From<[K; N]> for Vector<K, N> {
    fn from(array: [K; N]) -> Self {
        Vector::new(array)
    }
}
