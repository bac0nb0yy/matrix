pub struct Vector<K> {
    data: Vec<K>,
    size: usize,
}

#[derive(Debug)]
struct VectorSizeMismatch;

impl std::fmt::Display for VectorSizeMismatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector size mismatch")
    }
}

impl<K: std::fmt::Display> std::fmt::Display for Vector<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in &self.data[..self.size] {
            writeln!(f, "[{:.1}]", item)?;
        }
        Ok(())
    }
}

impl<K> Vector<K>
where
    K: Copy + std::ops::Add<Output = K> + std::ops::Sub<Output = K> + std::ops::MulAssign,
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
        if self.size != v.size {
            panic!("{}", VectorSizeMismatch);
        }
    }

    fn new(data: Vec<K>, size: Option<usize>) -> Self {
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
        self.data.iter_mut().for_each(|v| *v *= a);
    }
}

impl<K, const N: usize> From<[K; N]> for Vector<K>
where
    K: Copy + std::ops::Add<Output = K> + std::ops::Sub<Output = K> + std::ops::MulAssign,
{
    fn from(array: [K; N]) -> Self {
        Vector::new(Vec::from(array), Some(N))
    }
}