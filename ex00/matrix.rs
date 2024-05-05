use rand::distributions::Standard;
use rand::prelude::*;
use std::default::Default;
use std::fmt;
use std::ops::{Add, Mul, Sub};

pub struct Matrix<K> {
    data: Vec<Vec<K>>,
    rows: usize,
    cols: usize,
}

impl<K: fmt::Display> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.data[..self.rows] {
            write!(f, "[")?;
            for (i, item) in row.iter().take(self.cols).enumerate() {
                if i != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:.3}", item)?;
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}

impl<K: Add<Output = K> + Sub<Output = K> + Mul<Output = K> + Copy + Default + fmt::Display> Add
    for Matrix<K>
where
    Standard: rand::distributions::Distribution<K>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.check_size(&rhs);

        let new_matrix: Vec<Vec<K>> = self
            .data
            .iter()
            .zip(&rhs.data)
            .map(|(row1, row2)| row1.iter().zip(row2).map(|(&a, &b)| a + b).collect())
            .collect();

        Matrix::new(new_matrix, Some(self.rows), Some(self.cols))
    }
}

impl<K: Add<Output = K> + Sub<Output = K> + Mul<Output = K> + Copy + Default + fmt::Display> Sub
    for Matrix<K>
where
    Standard: rand::distributions::Distribution<K>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.check_size(&rhs);

        let new_matrix: Vec<Vec<K>> = self
            .data
            .iter()
            .zip(&rhs.data)
            .map(|(row1, row2)| row1.iter().zip(row2).map(|(&a, &b)| a - b).collect())
            .collect();

        Matrix::new(new_matrix, Some(self.rows), Some(self.cols))
    }
}

impl<K: Add<Output = K> + Sub<Output = K> + Mul<Output = K> + Copy + Default + fmt::Display> Mul
    for Matrix<K>
where
    Standard: rand::distributions::Distribution<K>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.check_size(&rhs);

        let new_matrix: Vec<Vec<K>> = self
            .data
            .iter()
            .zip(&rhs.data)
            .map(|(row1, row2)| row1.iter().zip(row2).map(|(&a, &b)| a * b).collect())
            .collect();

        Matrix::new(new_matrix, Some(self.rows), Some(self.cols))
    }
}

impl<K> Matrix<K>
where
    K: Copy + Add<Output = K> + Sub<Output = K> + Mul<Output = K> + fmt::Display,
    Standard: rand::distributions::Distribution<K>,
{
    fn operate<F>(&mut self, v: &Matrix<K>, op: F)
    where
        F: Fn(K, K) -> K,
    {
        self.check_size(v);

        self.data
            .iter_mut()
            .zip(&v.data)
            .for_each(|(a_row, b_row)| {
                a_row
                    .iter_mut()
                    .zip(b_row)
                    .for_each(|(a, b)| *a = op(*a, *b));
            });
    }

    fn check_size(&self, v: &Matrix<K>) {
        assert!(
            self.rows == v.rows && self.cols == v.cols,
            "Matrix size mismatch"
        );
    }

    pub fn new(data: Vec<Vec<K>>, rows: Option<usize>, cols: Option<usize>) -> Self {
        let rows: usize = rows.unwrap_or(data.len());
        let cols: usize = cols.unwrap_or(if rows > 0 { data[0].len() } else { 0 });
        Matrix { data, rows, cols }
    }

    pub fn add(&mut self, v: &Matrix<K>) {
        self.operate(v, |a, b| a + b);
    }

    pub fn sub(&mut self, v: &Matrix<K>) {
        self.operate(v, |a, b| a - b);
    }

    pub fn scl(&mut self, a: K) {
        self.data
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|v| *v = *v * a));
    }

    fn generate_random_matrix(cols: usize, rows: usize) -> Matrix<K> {
        let mut rng = rand::thread_rng();
        let data: Vec<Vec<K>> = (0..rows)
            .map(|_| (0..cols).map(|_| rng.gen()).collect())
            .collect();
        Matrix::new(data, Some(rows), Some(cols))
    }

    fn generate_random_operation(&mut self) {
        let mut rng = rand::thread_rng();
        let operation = rng.gen_range(0..3);
        let rows = self.rows;
        let cols = self.cols;

        print!("\n{}", self);
        match operation {
            0 => {
                let random_matrix = Self::generate_random_matrix(rows, cols);
                println!("+\n{}=", random_matrix);
                self.add(&random_matrix);
            }
            1 => {
                let random_matrix = Self::generate_random_matrix(rows, cols);
                println!("-\n{}=", random_matrix);
                self.sub(&random_matrix);
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

    pub fn run_random_tests(&mut self, num_tests: usize) {
        for _ in 0..num_tests {
            self.generate_random_operation();
        }
    }
}

impl<K, const M: usize, const N: usize> From<[[K; N]; M]> for Matrix<K>
where
    K: Copy + Add<Output = K> + Sub<Output = K> + Mul<Output = K> + fmt::Display,
    Standard: rand::distributions::Distribution<K>,
{
    fn from(array: [[K; N]; M]) -> Self {
        Matrix::new(
            array.iter().map(|row| Vec::from(*row)).collect(),
            Some(M),
            Some(N),
        )
    }
}
