use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Matrix<T, const W: usize, const H: usize> {
    inner: [[T; W]; H],
}

impl<T, const W: usize, const H: usize> Matrix<T, W, H>
where
    T: Default + Copy,
{
    pub fn empty() -> Matrix<T, W, H> {
        Matrix {
            inner: [[Default::default(); W]; H],
        }
    }
}

impl<T, const W: usize, const H: usize> Matrix<T, W, H> {
    pub fn all(&self) -> impl Iterator<Item = &T> + '_ {
        self.inner.iter().map(|row| row.iter()).flatten()
    }

    pub fn row(&self, i: usize) -> impl Iterator<Item = &T> + '_ {
        self.inner[i].iter()
    }

    pub fn col(&self, i: usize) -> impl Iterator<Item = &T> + '_ {
        self.inner.iter().map(move |row| &row[i])
    }
}

impl<T, const W: usize, const H: usize> Index<(usize, usize)> for Matrix<T, W, H> {
    type Output = T;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.inner[y][x]
    }
}

impl<T, const W: usize, const H: usize> IndexMut<(usize, usize)> for Matrix<T, W, H> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.inner[y][x]
    }
}
