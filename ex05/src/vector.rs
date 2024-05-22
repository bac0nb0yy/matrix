use crate::field::*;

use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone)]
pub struct Vector<K: Field> {
    data: Vec<K>,
    size: usize,
}

impl<K: Field + Display> Display for Vector<K> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[")?;
        for (index, item) in self.data.iter().enumerate() {
            if index != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:.3}", item)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl<K: Field> Add for Vector<K> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.check_size(&rhs);

        Vector::new(
            self.data
                .iter()
                .zip(&rhs.data)
                .map(|(a, b)| *a + *b)
                .collect(),
        )
    }
}

impl<K: Field> Add<K> for Vector<K> {
    type Output = Self;

    fn add(self, rhs: K) -> Self::Output {
        Vector::new(self.data.iter().map(|&a| a + rhs).collect())
    }
}

impl<K: Field> AddAssign<Vector<K>> for Vector<K> {
    fn add_assign(&mut self, rhs: Vector<K>) {
        assert_eq!(self.size, rhs.size, "Vector size mismatch");

        self.data
            .iter_mut()
            .zip(&rhs.data)
            .for_each(|(a, &b)| *a += b);
    }
}

impl<K: Field> AddAssign<K> for Vector<K> {
    fn add_assign(&mut self, rhs: K) {
        self.data.iter_mut().for_each(|a| *a += rhs);
    }
}

impl<K: Field> Sub for Vector<K> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.check_size(&rhs);

        Vector::new(
            self.data
                .iter()
                .zip(&rhs.data)
                .map(|(a, b)| *a - *b)
                .collect(),
        )
    }
}

impl<K: Field> Sub<K> for Vector<K> {
    type Output = Self;

    fn sub(self, rhs: K) -> Self::Output {
        Vector::new(self.data.iter().map(|&a| a - rhs).collect())
    }
}

impl<K: Field> SubAssign<Vector<K>> for Vector<K> {
    fn sub_assign(&mut self, rhs: Vector<K>) {
        assert_eq!(self.size, rhs.size, "Vector size mismatch");

        self.data
            .iter_mut()
            .zip(&rhs.data)
            .for_each(|(a, &b)| *a -= b);
    }
}

impl<K: Field> SubAssign<K> for Vector<K> {
    fn sub_assign(&mut self, rhs: K) {
        self.data.iter_mut().for_each(|a| *a -= rhs);
    }
}

impl<K: Field> Mul for Vector<K> {
    type Output = K;

    fn mul(self, rhs: Vector<K>) -> K {
        self.dot(&rhs)
    }
}

impl<K: Field> Mul<K> for Vector<K> {
    type Output = Self;

    fn mul(self, scalar: K) -> Self::Output {
        Vector::new(self.data.iter().map(|&a| a * scalar).collect())
    }
}

impl<K: Field> MulAssign<Vector<K>> for Vector<K> {
    fn mul_assign(&mut self, rhs: Vector<K>) {
        assert_eq!(self.size, rhs.size, "Vector size mismatch");

        self.data
            .iter_mut()
            .zip(&rhs.data)
            .for_each(|(a, &b)| *a *= b);
    }
}

impl<K: Field> MulAssign<K> for Vector<K> {
    fn mul_assign(&mut self, scl: K) {
        self.data.iter_mut().for_each(|a| *a *= scl);
    }
}

impl<K: Field> Div for Vector<K> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.check_size(&rhs);

        Vector::new(
            self.data
                .iter()
                .zip(&rhs.data)
                .map(|(a, b)| *a / *b)
                .collect(),
        )
    }
}

impl<K: Field> Div<K> for Vector<K> {
    type Output = Self;

    fn div(self, scalar: K) -> Self::Output {
        Vector::new(self.data.iter().map(|&a| a / scalar).collect())
    }
}

impl<K: Field> DivAssign<Vector<K>> for Vector<K> {
    fn div_assign(&mut self, rhs: Vector<K>) {
        assert_eq!(self.size, rhs.size, "Vector size mismatch");

        self.data
            .iter_mut()
            .zip(&rhs.data)
            .for_each(|(a, &b)| *a /= b);
    }
}

impl<K: Field> DivAssign<K> for Vector<K> {
    fn div_assign(&mut self, rhs: K) {
        self.data.iter_mut().for_each(|a| *a /= rhs);
    }
}

impl<K: Field> Neg for Vector<K> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector {
            data: self.data.into_iter().map(|x| -x).collect(),
            size: self.size,
        }
    }
}

impl<K: Field> Vector<K> {
    fn check_size(&self, v: &Vector<K>) {
        assert_eq!(self.size, v.size, "Vector size mismatch");
    }

    pub fn new(data: Vec<K>) -> Self {
        let size: usize = data.len();
        Vector { data, size }
    }

    pub fn norm(&self) -> f64 {
        self.dot(self).into().sqrt()
    }

    pub fn dot(&self, v: &Vector<K>) -> K {
        self.check_size(v);

        self.data
            .iter()
            .zip(&v.data)
            .fold(K::zero(), |acc, (&x, &y)| acc + x * y)
    }

    #[allow(dead_code)]
    pub fn data(&self) -> &Vec<K> {
        &self.data
    }

    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.size
    }
}

impl<K: Field, const N: usize> From<[K; N]> for Vector<K> {
    fn from(array: [K; N]) -> Self {
        Vector::new(Vec::from(array))
    }
}
