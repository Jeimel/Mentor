use super::vector::Vector;

#[derive(Copy, Clone)]
pub struct Matrix<const M: usize, const N: usize> {
    inner: [Vector<N>; M],
}

#[allow(dead_code)]
impl<const M: usize, const N: usize> Matrix<M, N> {
    pub fn transpose_mul(&self, vec: Vector<M>) -> Vector<N> {
        (0..N)
            .map(|i| {
                let mut value = 0.0;

                for j in 0..M {
                    value += self.inner[j][i] * vec[j]
                }

                value
            })
            .collect::<Vector<N>>()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Vector<N>> {
        self.inner.iter_mut()
    }

    pub fn rand(normal: bool) -> Self {
        Self {
            inner: [Vector::zeroed(); M].map(|_: Vector<N>| Vector::rand(normal)),
        }
    }

    pub const fn zeroed() -> Self {
        Self::from_raw([Vector::zeroed(); M])
    }

    pub const fn from_raw(inner: [Vector<N>; M]) -> Self {
        Self { inner }
    }
}

impl<const M: usize, const N: usize> std::ops::SubAssign<Matrix<M, N>> for Matrix<M, N> {
    fn sub_assign(&mut self, rhs: Matrix<M, N>) {
        for (u, v) in self.inner.iter_mut().zip(rhs.inner.iter()) {
            *u -= *v;
        }
    }
}

impl<const M: usize, const N: usize> std::ops::Mul<Vector<N>> for Matrix<M, N> {
    type Output = Vector<M>;

    fn mul(self, v: Vector<N>) -> Self::Output {
        self.inner.iter().map(|u| v.dot(u)).collect::<Vector<M>>()
    }
}

impl<const M: usize, const N: usize> std::ops::Mul<f32> for Matrix<M, N> {
    type Output = Matrix<M, N>;

    fn mul(mut self, rhs: f32) -> Self::Output {
        for i in self.inner.iter_mut() {
            *i *= rhs;
        }

        self
    }
}
