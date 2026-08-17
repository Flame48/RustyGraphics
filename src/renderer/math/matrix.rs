pub struct Matrix<const M: usize, const N: usize> {
    data: [[f32; N]; M],
}

impl<const M: usize, const N: usize> Matrix<M, N> {
    pub fn new() -> Self {
        Self { data: [[0.0; N]; M] }
    }

    pub fn from_data(data: [[f32; N]; M]) -> Self {
        Self { data }
    }

    pub fn get(&self, row: usize, col: usize) -> f32 {
        self.data[row][col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: f32) {
        self.data[row][col] = value;
    }

    fn apply(&self, other: &Matrix<M, N>, f: impl Fn(f32, f32) -> f32) -> Matrix<M, N> {
        let mut result = Matrix::<M, N>::new();
        for i in 0..M {
            for j in 0..N {
                result.data[i][j] = f(self.data[i][j], other.data[i][j]);
            }
        }
        result
    }

    fn apply_to(&mut self, other: &Matrix<M, N>, f: impl Fn(f32, f32) -> f32) {
        for i in 0..M {
            for j in 0..N {
                self.data[i][j] = f(self.data[i][j], other.data[i][j]);
            }
        }
    }

    pub fn mul<const S: usize>(&self, other: &Matrix<N, S>) -> Matrix<M, S> {
        let mut result = Matrix::<M, S>::new();
        for i in 0..M {
            for j in 0..S {
                let mut sum = 0.0;
                for k in 0..N {
                    sum += self.data[i][k] * other.data[k][j];
                }
                result.data[i][j] = sum;
            }
        }
        result
    }

    pub fn transpose(&self) -> Matrix<N, M> {
        let mut result = Matrix::<N, M>::new();
        for i in 0..M {
            for j in 0..N {
                result.data[j][i] = self.data[i][j];
            }
        }
        result
    }

    pub fn add(&self, other: &Matrix<M, N>) -> Matrix<M, N> {
        self.apply(other, |a, b| a + b)
    }

    pub fn sub(&self, other: &Matrix<M, N>) -> Matrix<M, N> {
        self.apply(other, |a, b| a - b)
    }

    pub fn add_to(&mut self, other: &Matrix<M, N>) {
        self.apply_to(other, |a, b| a + b)
    }

    pub fn sub_to(&mut self, other: &Matrix<M, N>) {
        self.apply_to(other, |a, b| a - b)
    }
}

impl<const M: usize, const N: usize> std::ops::Add for Matrix<M, N> {
    type Output = Matrix<M, N>;

    fn add(self, other: Matrix<M, N>) -> Matrix<M, N> {
        Matrix::add(&self, &other)
    }
}

impl<const M: usize, const N: usize> std::ops::AddAssign for Matrix<M, N> {
    fn add_assign(&mut self, other: Matrix<M, N>) {
        Matrix::add_to(self, &other);
    }
}

impl<const M: usize, const N: usize> std::ops::Sub for Matrix<M, N> {
    type Output = Matrix<M, N>;

    fn sub(self, other: Matrix<M, N>) -> Matrix<M, N> {
        Matrix::sub(&self, &other)
    }
}

impl<const M: usize, const N: usize> std::ops::SubAssign for Matrix<M, N> {
    fn sub_assign(&mut self, other: Matrix<M, N>) {
        Matrix::sub_to(self, &other);
    }
}

impl<const M: usize, const N: usize, const S: usize> std::ops::Mul<Matrix<N, S>> for Matrix<M, N> {
    type Output = Matrix<M, S>;

    fn mul(self, other: Matrix<N, S>) -> Matrix<M, S> {
        Matrix::mul(&self, &other)
    }
}

impl<const M: usize, const N: usize> std::ops::Index<(usize, usize)> for Matrix<M, N> {
    type Output = f32;

    fn index(&self, (row, col): (usize, usize)) -> &f32 {
        &self.data[row][col]
    }
}

impl<const M: usize, const N: usize> std::ops::IndexMut<(usize, usize)> for Matrix<M, N> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut f32 {
        &mut self.data[row][col]
    }
}
