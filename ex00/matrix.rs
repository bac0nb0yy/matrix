use std::fmt;

pub struct Matrix<K> {
    data: Vec<Vec<K>>,
    rows: usize,
    cols: usize,
}

#[derive(Debug)]
struct MatrixSizeMismatch;

impl<K: fmt::Display> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.data[..self.rows] {
            write!(f, "[")?;
            for (i, item) in row.iter().take(self.cols).enumerate() {
                if i != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:.1}", item)?;
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}

impl std::fmt::Display for MatrixSizeMismatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Matrix size mismatch")
    }
}

impl<K> Matrix<K>
where
    K: Copy + std::ops::Add<Output = K> + std::ops::Sub<Output = K> + std::ops::MulAssign,
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
        if self.rows != v.rows || self.cols != v.cols {
            panic!("{}", MatrixSizeMismatch);
        }
    }

    fn new(data: Vec<Vec<K>>, rows: Option<usize>, cols: Option<usize>) -> Self {
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
            .for_each(|row| row.iter_mut().for_each(|v| *v *= a));
    }
}

impl<K, const M: usize, const N: usize> From<[[K; N]; M]> for Matrix<K>
where
    K: Copy + std::ops::Add<Output = K> + std::ops::Sub<Output = K> + std::ops::MulAssign,
{
    fn from(array: [[K; N]; M]) -> Self {
        Matrix::new(
            array.iter().map(|row| Vec::from(*row)).collect(),
            Some(M),
            Some(N),
        )
    }
}
