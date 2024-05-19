use std::default::Default;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone)]
pub struct Matrix<K> {
    data: Vec<Vec<K>>,
    rows: usize,
    cols: usize,
}

impl<K: Display> Display for Matrix<K> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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

impl<K: Add<Output = K> + Sub<Output = K> + Mul<Output = K> + Copy + Default> Add for Matrix<K> {
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

impl<K> AddAssign<Matrix<K>> for Matrix<K>
where
    K: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: Matrix<K>) {
        assert!(
            self.rows == rhs.rows && self.cols == rhs.cols,
            "Matrix size mismatch"
        );

        self.data
            .iter_mut()
            .zip(&rhs.data)
            .for_each(|(row1, row2)| row1.iter_mut().zip(row2).for_each(|(a, &b)| *a += b));
    }
}

impl<K: Add<Output = K> + Sub<Output = K> + Mul<Output = K> + Copy + Default> Sub for Matrix<K> {
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

impl<K> SubAssign<Matrix<K>> for Matrix<K>
where
    K: SubAssign + Copy,
{
    fn sub_assign(&mut self, rhs: Matrix<K>) {
        assert!(
            self.rows == rhs.rows && self.cols == rhs.cols,
            "Matrix size mismatch"
        );

        self.data
            .iter_mut()
            .zip(&rhs.data)
            .for_each(|(row1, row2)| row1.iter_mut().zip(row2).for_each(|(a, &b)| *a -= b));
    }
}

impl<K: Add<Output = K> + Sub<Output = K> + Mul<Output = K> + Copy + Default> Mul for Matrix<K> {
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

impl<K> MulAssign<K> for Matrix<K>
where
    K: MulAssign + Copy,
{
    fn mul_assign(&mut self, scl: K) {
        self.data
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|v| *v *= scl));
    }
}

impl<K> Matrix<K>
where
    K: Add<Output = K> + Sub<Output = K> + Mul<Output = K> + Copy + Default,
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

    #[allow(dead_code)]
    pub fn get_data(&self) -> &Vec<Vec<K>> {
        &self.data
    }

    #[allow(dead_code)]
    pub fn get_rows(&self) -> usize {
        self.rows
    }

    #[allow(dead_code)]
    pub fn get_cols(&self) -> usize {
        self.cols
    }
}

impl<K, const M: usize, const N: usize> From<[[K; N]; M]> for Matrix<K>
where
    K: Add<Output = K> + Sub<Output = K> + Mul<Output = K> + Copy + Default,
{
    fn from(array: [[K; N]; M]) -> Self {
        Matrix::new(
            array.iter().map(|row| Vec::from(*row)).collect(),
            Some(M),
            Some(N),
        )
    }
}
