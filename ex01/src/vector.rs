use rand::distributions::Standard;
use rand::prelude::*;
use std::default::Default;
use std::fmt;
use std::ops::{Add, Mul, Sub};

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

    pub fn add(&mut self, v: &Vector<K>) {
        self.operate(v, |a, b| a + b);
    }

    pub fn sub(&mut self, v: &Vector<K>) {
        self.operate(v, |a, b| a - b);
    }

    pub fn scl(&mut self, a: K) {
        self.data.iter_mut().for_each(|v| *v = *v * a);
    }

    pub fn linear_combination(u: &[Vector<K>], coefs: &[K]) -> Vector<K> {
        assert_eq!(
            u.len(),
            coefs.len(),
            "Number of vectors and coefficients must match"
        );

        let result = u.iter().zip(coefs.iter()).fold(
            Vector::new(vec![K::default(); u[0].data.len()], Some(u[0].data.len())),
            |mut acc, (vector, coef)| {
                acc.add_scaled(vector, *coef);
                acc
            },
        );

        result
    }

    fn add_scaled(&mut self, v: &Vector<K>, scalar: K) {
        self.check_size(v);

        self.data
            .iter_mut()
            .zip(&v.data)
            .for_each(|(a, b)| *a = *a + scalar * *b);
    }

    #[allow(dead_code)]
    fn generate_random_vector(size: usize) -> Vector<K> {
        let mut rng = rand::thread_rng();
        let data: Vec<K> = (0..size).map(|_| rng.gen()).collect();
        Vector::new(data, Some(size))
    }

    #[allow(dead_code)]
    fn generate_random_operation(&mut self) {
        let mut rng = rand::thread_rng();
        let operation = rng.gen_range(0..3);
        let size = self.size;

        print!("\n{}", self);
        match operation {
            0 => {
                let random_vector = Self::generate_random_vector(size);
                println!("+\n{}=", random_vector);
                self.add(&random_vector);
            }
            1 => {
                let random_vector = Self::generate_random_vector(size);
                println!("-\n{}=", random_vector);
                self.sub(&random_vector);
            }
            2 => {
                let scalar: K = rng.gen();
                println!("*\n{}\n=", scalar);
                self.scl(scalar);
            }
            _ => unreachable!(),
        }
        println!("{}\n\n", self);
    }

    #[allow(dead_code)]
    pub fn run_random_tests(&mut self, num_tests: usize) {
        for _ in 0..num_tests {
            self.generate_random_operation();
        }
    }
}

impl<K, const N: usize> From<[K; N]> for Vector<K>
where
    K: Copy
        + Add<Output = K>
        + Sub<Output = K>
        + Mul<Output = K>
        + fmt::Display
        + std::default::Default,
    Standard: rand::distributions::Distribution<K>,
{
    fn from(array: [K; N]) -> Self {
        Vector::new(Vec::from(array), Some(N))
    }
}
