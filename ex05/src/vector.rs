use std::default::Default;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone)]
pub struct Vector<K> {
    data: Vec<K>,
    size: usize,
}

impl<K: Display> Display for Vector<K> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for item in &self.data[..self.size] {
            writeln!(f, "[{:.3}]", item)?;
        }
        Ok(())
    }
}

impl<K: Add<Output = K> + Sub<Output = K> + Mul<Output = K> + Copy + Default> Add for Vector<K>
where
    f64: From<K>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.check_size(&rhs);

        let new_vector: Vec<K> = self
            .data
            .iter()
            .zip(&rhs.data)
            .map(|(a, b)| *a + *b)
            .collect();

        Vector::new(new_vector, Some(self.size))
    }
}

impl<K> AddAssign<Vector<K>> for Vector<K>
where
    K: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: Vector<K>) {
        assert_eq!(self.size, rhs.size, "Vector size mismatch");

        self.data
            .iter_mut()
            .zip(&rhs.data)
            .for_each(|(a, &b)| *a += b);
    }
}

impl<K: Add<Output = K> + Sub<Output = K> + Mul<Output = K> + Copy + Default> Sub for Vector<K>
where
    f64: From<K>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.check_size(&rhs);

        let new_vector: Vec<K> = self
            .data
            .iter()
            .zip(&rhs.data)
            .map(|(a, b)| *a - *b)
            .collect();

        Vector::new(new_vector, Some(self.size))
    }
}

impl<K> SubAssign<Vector<K>> for Vector<K>
where
    K: SubAssign + Copy,
{
    fn sub_assign(&mut self, rhs: Vector<K>) {
        assert_eq!(self.size, rhs.size, "Vector size mismatch");

        self.data
            .iter_mut()
            .zip(&rhs.data)
            .for_each(|(a, &b)| *a -= b);
    }
}

impl<K: Add<Output = K> + Sub<Output = K> + Mul<Output = K> + Copy + Default> Mul for Vector<K>
where
    f64: From<K>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.check_size(&rhs);

        let new_vector: Vec<K> = self
            .data
            .iter()
            .zip(&rhs.data)
            .map(|(a, b)| *a * *b)
            .collect();

        Vector::new(new_vector, Some(self.size))
    }
}

impl<K> MulAssign<K> for Vector<K>
where
    K: MulAssign + Copy,
{
    fn mul_assign(&mut self, scl: K) {
        self.data.iter_mut().for_each(|a| *a *= scl);
    }
}

impl<K: Mul<Output = K> + Sub<Output = K> + Add<Output = K> + Into<f64> + Copy + Default> Mul<K>
    for Vector<K>
{
    type Output = Self;

    fn mul(self, scalar: K) -> Self::Output {
        let new_vector: Vec<K> = self.data.iter().map(|&a| a * scalar).collect();
        Vector::new(new_vector, Some(self.size))
    }
}

impl<K> Vector<K>
where
    K: Add<Output = K> + Sub<Output = K> + Mul<Output = K> + Into<f64> + Copy + Default,
{
    fn check_size(&self, v: &Vector<K>) {
        assert_eq!(self.size, v.size, "Vector size mismatch");
    }

    pub fn new(data: Vec<K>, size: Option<usize>) -> Self {
        let size: usize = size.unwrap_or(data.len());
        Vector { data, size }
    }

    #[allow(dead_code)]
    pub fn norm(&self) -> f64 {
        self.data
            .iter()
            .fold(f64::default(), |acc, &x| acc + (x.into() * x.into()))
            .sqrt()
    }

    #[allow(dead_code)]
    pub fn dot(&self, v: &Vector<K>) -> K {
        self.check_size(v);

        self.data
            .iter()
            .zip(&v.data)
            .fold(K::default(), |acc, (&x, &y)| acc + x * y)
    }

    #[allow(dead_code)]
    pub fn get_data(&self) -> &Vec<K> {
        &self.data
    }

    #[allow(dead_code)]
    pub fn get_size(&self) -> usize {
        self.size
    }
}

impl<K, const N: usize> From<[K; N]> for Vector<K>
where
    K: Add<Output = K> + Sub<Output = K> + Mul<Output = K> + Into<f64> + Copy + Default,
{
    fn from(array: [K; N]) -> Self {
        Vector::new(Vec::from(array), Some(N))
    }
}
