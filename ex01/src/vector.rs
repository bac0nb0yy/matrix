use rand::distributions::Standard;
use std::default::Default;
use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone)]
pub struct Vector<K> {
    data: Vec<K>,
    size: usize,
}

impl<K: fmt::Display> fmt::Display for Vector<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for item in &self.data[..self.size] {
            writeln!(f, "[{:.3}]", item)?;
        }
        Ok(())
    }
}

impl<K: Add<Output = K> + Sub<Output = K> + Mul<Output = K> + Copy + Default + fmt::Display> Add
    for Vector<K>
where
    Standard: rand::distributions::Distribution<K>,
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

impl<K: Add<Output = K> + Sub<Output = K> + Mul<Output = K> + Copy + Default + fmt::Display> Sub
    for Vector<K>
where
    Standard: rand::distributions::Distribution<K>,
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

impl<K: Add<Output = K> + Sub<Output = K> + Mul<Output = K> + Copy + Default + fmt::Display> Mul
    for Vector<K>
where
    Standard: rand::distributions::Distribution<K>,
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

impl<K> Vector<K>
where
    K: Copy + Add<Output = K> + Sub<Output = K> + Mul<Output = K> + fmt::Display + Default,
    Standard: rand::distributions::Distribution<K>,
{
    fn operate<F>(&mut self, v: &Vector<K>, op: F)
    where
        F: Fn(K, K) -> K,
    {
        self.check_size(v);

        self.data
            .iter_mut()
            .zip(&v.data)
            .for_each(|(a, b)| *a = op(*a, *b));
    }

    fn check_size(&self, v: &Vector<K>) {
        assert_eq!(self.size, v.size, "Vector size mismatch");
    }

    pub fn new(data: Vec<K>, size: Option<usize>) -> Self {
        let size: usize = size.unwrap_or(data.len());
        Vector { data, size }
    }

    #[allow(dead_code)]
    pub fn add(&mut self, v: &Vector<K>) {
        self.operate(v, |a, b| a + b);
    }

    #[allow(dead_code)]
    pub fn sub(&mut self, v: &Vector<K>) {
        self.operate(v, |a, b| a - b);
    }

    #[allow(dead_code)]
    pub fn scl(&mut self, a: K) {
        self.data.iter_mut().for_each(|v| *v = *v * a);
    }

    pub fn linear_combination(u: &[Vector<K>], coefs: &[K]) -> Vector<K> {
        assert_eq!(
            u.len(),
            coefs.len(),
            "Number of vectors and coefficients must match"
        );

        u.iter().zip(coefs.iter()).fold(
            Vector::new(vec![K::default(); u[0].data.len()], Some(u[0].data.len())),
            |mut acc, (vector, coef)| {
                acc.data
                    .iter_mut()
                    .zip(&vector.data)
                    .for_each(|(a, b)| *a = *a + *coef * *b);
                acc
            },
        )
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
    K: Copy + Add<Output = K> + Sub<Output = K> + Mul<Output = K> + fmt::Display + Default,
    Standard: rand::distributions::Distribution<K>,
{
    fn from(array: [K; N]) -> Self {
        Vector::new(Vec::from(array), Some(N))
    }
}