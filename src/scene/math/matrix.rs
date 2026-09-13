use crate::scene::math::transforms::Transform;

#[derive(Clone, Copy)]
pub struct Matrix<const M: usize, const N: usize> {
    pub data: [[f32; N]; M],
}

pub type ColMat<const M: usize> = Matrix<M, 1>;
pub type RowMat<const N: usize> = Matrix<1, N>;
pub type SqMat<const S: usize> = Matrix<S, S>;

pub type Quaternion = RowMat<4>;

// MARK: General Matrix

impl<const M: usize, const N: usize> Matrix<M, N> {
    pub const fn new() -> Self {
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

    pub fn row(&self, row: usize) -> [f32; N] {
        return self.data[row];
    }

    pub fn col(&self, col: usize) -> [f32; M] {
        let mut res = [0.0; M];
        for i in 0..M {
            res[i] = self.data[i][col];
        }
        res
    }

    pub fn row_mat(&self, row: usize) -> RowMat<N> {
        RowMat::from_data([self.row(row)])
    }

    pub fn col_mat(&self, col: usize) -> ColMat<M> {
        let mut res: [[f32; 1]; M] = [[0.0; 1]; M];
        for i in 0..M {
            res[i] = [self.data[i][col]];
        }
        ColMat::from_data(res)
    }

    pub fn apply(&self, other: &Matrix<M, N>, f: impl Fn(f32, f32) -> f32) -> Self {
        let mut result = Self::new();
        for i in 0..M {
            for j in 0..N {
                result.data[i][j] = f(self.data[i][j], other.data[i][j]);
            }
        }
        result
    }

    pub fn apply_to(&mut self, other: &Matrix<M, N>, f: impl Fn(f32, f32) -> f32) {
        for i in 0..M {
            for j in 0..N {
                self.data[i][j] = f(self.data[i][j], other.data[i][j]);
            }
        }
    }

    pub fn map(&self, f: impl Fn(f32) -> f32) -> Self {
        let mut result = Self::new();
        for i in 0..M {
            for j in 0..N {
                result.data[i][j] = f(self.data[i][j]);
            }
        }
        result
    }

    pub fn map_to(&mut self, f: impl Fn(f32) -> f32) {
        for i in 0..M {
            for j in 0..N {
                self.data[i][j] = f(self.data[i][j]);
            }
        }
    }

    pub fn sum(&self) -> f32 {
        self.data.as_flattened().iter().sum()
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

    pub fn scalar_mul(&self, by: f32) -> Self {
        self.map(|x| x * by)
    }

    pub fn scalar_mul_to(&mut self, by: f32) {
        self.map_to(|x| x * by)
    }

    pub fn scalar_div(&self, by: f32) -> Self {
        self.map(|x| x / by)
    }

    pub fn scalar_div_to(&mut self, by: f32) {
        self.map_to(|x| x / by)
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

// MARK: Matrix Operations

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

impl<const M: usize, const N: usize> std::ops::Mul<f32> for Matrix<M, N> {
    type Output = Matrix<M, N>;

    fn mul(self, other: f32) -> Matrix<M, N> {
        Matrix::scalar_mul(&self, other)
    }
}

impl<const M: usize, const N: usize> std::ops::MulAssign<f32> for Matrix<M, N> {
    fn mul_assign(&mut self, other: f32) {
        self.scalar_mul_to(other);
    }
}

impl<const M: usize, const N: usize> std::ops::Div<f32> for Matrix<M, N> {
    type Output = Matrix<M, N>;

    fn div(self, other: f32) -> Matrix<M, N> {
        Matrix::scalar_div(&self, other)
    }
}

impl<const M: usize, const N: usize> std::ops::DivAssign<f32> for Matrix<M, N> {
    fn div_assign(&mut self, other: f32) {
        Matrix::scalar_div_to(self, other);
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

impl<const M: usize, const N: usize> std::ops::Neg for Matrix<M, N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.map(|x| -x)
    }
}

// MARK: Square Matrix
impl<const M: usize> SqMat<M> {
    pub fn identity() -> Self {
        let mut result = SqMat::<M>::new();
        for i in 0..M {
            result.data[i][i] = 1.0;
        }
        result
    }
}

// MARK: Row x 4 Matrix
impl<const M: usize> Matrix<M, 4> {
    pub fn normalize_homogenous(&self) -> Self {
        let mut result = Self::new();
        for i in 0..M {
            let w = self.data[i][3];
            for j in 0..4 {
                result.data[i][j] = self.data[i][j] / w;
            }
        }
        result
    }

    pub fn normalize_homogenous_mut(&mut self) {
        for i in 0..M {
            let w = self.data[i][3];
            for j in 0..4 {
                self.data[i][j] /= w;
            }
        }
    }

    pub fn to_uniform(&self) -> Matrix<M, 3> {
        let mut result = Matrix::<M, 3>::new();
        for i in 0..M {
            let w = self.data[i][3];
            if w.abs() <= 1e-6 {
                for j in 0..3 {
                    result.data[i][j] = self.data[i][j];
                }
                continue;
            }
            for j in 0..3 {
                result.data[i][j] = self.data[i][j] / w;
            }
        }
        result
    }
}

// MARK: Column Matrix
impl<const M: usize> ColMat<M> {
    pub fn serial_col(&self) -> [f32; M] {
        self.col(0)
    }

    pub fn mag_col(&self) -> f32 {
        self.map(|x| x * x)
            .sum()
            .sqrt()
    }

    pub fn norm_col(&self) -> Self {
        let mag = self.mag_col();
        self.map(|x| x / mag)
    }

    pub fn norm_col_to(&mut self) {
        let mag = self.mag_col();
        self.map_to(|x| x / mag)
    }
}

// MARK: Row Matrix
impl<const N: usize> RowMat<N> {
    pub fn serial_row(&self) -> [f32; N] {
        self.row(0)
    }

    pub fn mag_row(&self) -> f32 {
        self.map(|x| x * x)
            .sum()
            .sqrt()
    }

    pub fn norm_row(&self) -> Self {
        let mag = self.mag_row();
        self.map(|x| x / mag)
    }

    pub fn norm_row_to(&mut self) {
        let mag = self.mag_row();
        self.map_to(|x| x / mag)
    }

    pub fn dot(&self, other: Self) -> f32 {
        let mut acc = 0.0;
        for i in 0..N {
            acc += self.data[0][i] * other.data[0][i];
        }
        acc
    }
}

// MARK: Itemized Matrix
impl SqMat<1> {
    pub fn item(&self) -> f32 {
        self.get(0, 0)
    }
}

// MARK: 4x4 Transform Matrix
impl SqMat<4> {
    pub fn scale(s: RowMat<3>) -> Self {
        Self::from_data([
            [s.x(), 0.0, 0.0, 0.0],
            [0.0, s.y(), 0.0, 0.0],
            [0.0, 0.0, s.z(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
    pub fn translation(p: RowMat<3>) -> Self {
        Self::from_data([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [p.x(), p.y(), p.z(), 1.0],
        ])
    }

    pub fn rotation(q: Quaternion) -> Self {
        let (w, x, y, z) = q.wxyz();
        let (x2, y2, z2) = (x + x, y + y, z + z);
        let (xx, xy, xz) = (x * x2, x * y2, x * z2);
        let (yy, yz, zz) = (y * y2, y * z2, z * z2);
        let (wx, wy, wz) = (w * x2, w * y2, w * z2);

        Self::from_data([
            [1.0 - (yy + zz), xy + wz, xz - wy, 0.0],
            [xy - wz, 1.0 - (xx + zz), yz + wx, 0.0],
            [xz + wy, yz - wx, 1.0 - (xx + yy), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn scale_inv(s: RowMat<3>) -> Self {
        Self::from_data([
            [1.0 / s.x(), 0.0, 0.0, 0.0],
            [0.0, 1.0 / s.y(), 0.0, 0.0],
            [0.0, 0.0, 1.0 / s.z(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
    pub fn translation_inv(p: RowMat<3>) -> Self {
        Self::from_data([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [-p.x(), -p.y(), -p.z(), 1.0],
        ])
    }

    pub fn rotation_inv(q: Quaternion) -> Self {
        let (w, x, y, z) = q.wxyz();
        Self::rotation(Quaternion::from_data([[w, -x, -y, -z]]))
    }
}

// MARK: XYZ Row Matrix
impl RowMat<2> {
    #[inline(always)]
    pub fn x(&self) -> f32 {
        self.data[0][0]
    }
    #[inline(always)]
    pub fn y(&self) -> f32 {
        self.data[0][1]
    }
}

impl RowMat<3> {
    #[inline(always)]
    pub fn x(&self) -> f32 {
        self.data[0][0]
    }
    #[inline(always)]
    pub fn y(&self) -> f32 {
        self.data[0][1]
    }
    #[inline(always)]
    pub fn z(&self) -> f32 {
        self.data[0][2]
    }

    pub fn cross(&self, other: Self) -> Self {
        Self::from_data([
            [
                self.y() * other.z() - self.z() * other.y(),
                self.z() * other.x() - self.x() * other.z(),
                self.x() * other.y() - self.y() * other.x(),
            ],
        ])
    }

    pub fn axis_x() -> Self {
        Self::from_data([[1.0, 0.0, 0.0]])
    }

    pub fn axis_y() -> Self {
        Self::from_data([[0.0, 1.0, 0.0]])
    }

    pub fn axis_z() -> Self {
        Self::from_data([[0.0, 0.0, 1.0]])
    }

    pub fn to_homogenous(&self) -> RowMat<4> {
        RowMat::<4>::from_data([[self.x(), self.y(), self.z(), 1.0]])
    }

    pub fn transform(&self, by: Transform) -> Self {
        (self.to_homogenous() * by.forward).to_uniform()
    }

    pub fn rotate_by_quaternion(&self, by: Quaternion) -> Self {
        self.transform(Transform::rotation(by))
    }
}

// MARK: Quaternion
impl Quaternion {
    #[inline(always)]
    pub fn w(&self) -> f32 {
        self.data[0][0]
    }

    #[inline(always)]
    pub fn x(&self) -> f32 {
        self.data[0][1]
    }

    #[inline(always)]
    pub fn y(&self) -> f32 {
        self.data[0][2]
    }

    #[inline(always)]
    pub fn z(&self) -> f32 {
        self.data[0][3]
    }

    #[inline(always)]
    pub fn wxyz(&self) -> (f32, f32, f32, f32) {
        (self.w(), self.x(), self.y(), self.z())
    }

    pub fn to_quaternion_matrix(&self) -> SqMat<4> {
        let (w, x, y, z) = self.wxyz();
        SqMat::<4>::from_data([
            [w, x, y, z],
            [-x, w, -z, y],
            [-y, z, w, -x],
            [-z, -y, x, w],
        ])
    }

    pub fn hamiltonion_quaternion_mul(&self, other: &Self) -> Self {
        self.mul(&other.to_quaternion_matrix())
    }

    pub fn hamiltonion_quaternion_mul_mut(&mut self, other: &Self) {
        self.clone_from(&self.mul(&other.to_quaternion_matrix()));
    }

    pub fn rotate_local(&self, axis: &RowMat<3>, by: f32) -> Self {
        let axis = axis.norm_row();
        let half_theta = by * 0.5;
        let q_ax = Quaternion::from_data([
            [
                half_theta.cos(),
                axis.x() * half_theta.sin(),
                axis.y() * half_theta.sin(),
                axis.z() * half_theta.sin(),
            ],
        ]);
        return self.hamiltonion_quaternion_mul(&q_ax).norm_row();
    }

    pub fn rotate_local_mut(&mut self, axis: &RowMat<3>, by: f32) {
        let axis = axis.norm_row();
        let half_theta = by * 0.5;
        let q_ax = Quaternion::from_data([
            [
                half_theta.cos(),
                axis.x() * half_theta.sin(),
                axis.y() * half_theta.sin(),
                axis.z() * half_theta.sin(),
            ],
        ]);
        self.hamiltonion_quaternion_mul_mut(&q_ax);
        self.norm_row_to();
    }

    pub fn rotate_global(&self, axis: &RowMat<3>, by: f32) -> Self {
        let axis = axis.norm_row();
        let half_theta = by * 0.5;
        let q_ax = Quaternion::from_data([
            [
                half_theta.cos(),
                axis.x() * half_theta.sin(),
                axis.y() * half_theta.sin(),
                axis.z() * half_theta.sin(),
            ],
        ]);
        return q_ax.hamiltonion_quaternion_mul(self).norm_row();
    }

    pub fn rotate_global_mut(&mut self, axis: &RowMat<3>, by: f32) {
        let axis = axis.norm_row();
        let half_theta = by * 0.5;
        let mut q_ax = Quaternion::from_data([
            [
                half_theta.cos(),
                axis.x() * half_theta.sin(),
                axis.y() * half_theta.sin(),
                axis.z() * half_theta.sin(),
            ],
        ]);
        q_ax.hamiltonion_quaternion_mul_mut(self);
        self.clone_from(&q_ax);
        self.norm_row_to();
    }
}
