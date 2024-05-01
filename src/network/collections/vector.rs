use rand::{rngs::ThreadRng, thread_rng};
use rand_distr::{Distribution, Normal, Uniform};

use crate::network::layer::activation::Activation;

#[derive(Copy, Clone)]
pub struct Vector<const N: usize> {
    inner: [f32; N],
}

impl<const N: usize> Vector<N> {
    pub fn activate<T: Activation>(mut self) -> Self {
        for i in self.inner.iter_mut() {
            *i = T::activate(*i);
        }

        self
    }

    pub fn derive<T: Activation>(mut self) -> Self {
        for i in self.inner.iter_mut() {
            *i = T::derive(*i);
        }

        self
    }

    pub fn dot(&self, other: &Vector<N>) -> f32 {
        let mut value = 0.0;
        for (&i, &j) in self.inner.iter().zip(other.inner.iter()) {
            value += i * j;
        }

        value
    }

    pub fn rand(normal: bool) -> Self {
        enum Dist {
            Normal(Normal<f32>),
            Uniform(Uniform<f32>),
        }

        impl Dist {
            fn new(stdev: f32, normal: bool) -> Self {
                if normal {
                    Self::Normal(Normal::new(0.0, stdev).unwrap())
                } else {
                    Self::Uniform(Uniform::new(-stdev, stdev))
                }
            }

            fn sample(&self, rng: &mut ThreadRng) -> f32 {
                match self {
                    Dist::Normal(x) => x.sample(rng),
                    Dist::Uniform(x) => x.sample(rng),
                }
            }
        }

        let mut rng = thread_rng();

        let stdev = (1.0 / N as f32).sqrt();
        let dist = Dist::new(stdev, normal);

        Self {
            inner: [0f32; N].map(|_| dist.sample(&mut rng)),
        }
    }

    pub const fn zeroed() -> Self {
        Self::from_raw([0.0; N])
    }

    pub const fn from_raw(inner: [f32; N]) -> Self {
        Self { inner }
    }
}

impl<const N: usize> std::ops::Index<usize> for Vector<N> {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl<const N: usize> std::ops::IndexMut<usize> for Vector<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

impl<const N: usize> std::ops::Add<Vector<N>> for Vector<N> {
    type Output = Vector<N>;

    fn add(mut self, rhs: Vector<N>) -> Self::Output {
        for (i, j) in self.inner.iter_mut().zip(rhs.inner.iter()) {
            *i += *j;
        }

        self
    }
}

impl<const N: usize> std::ops::AddAssign<Vector<N>> for Vector<N> {
    fn add_assign(&mut self, rhs: Vector<N>) {
        for (i, j) in self.inner.iter_mut().zip(rhs.inner.iter()) {
            *i += *j;
        }
    }
}

impl<const N: usize> std::ops::SubAssign<Vector<N>> for Vector<N> {
    fn sub_assign(&mut self, rhs: Vector<N>) {
        for (i, j) in self.inner.iter_mut().zip(rhs.inner.iter()) {
            *i -= *j;
        }
    }
}

impl<const N: usize> std::ops::Mul<Vector<N>> for Vector<N> {
    type Output = Vector<N>;

    fn mul(mut self, rhs: Vector<N>) -> Self::Output {
        for (i, j) in self.inner.iter_mut().zip(rhs.inner.iter()) {
            *i *= *j;
        }

        self
    }
}

impl<const N: usize> std::ops::Mul<f32> for Vector<N> {
    type Output = Vector<N>;

    fn mul(mut self, rhs: f32) -> Self::Output {
        for i in self.inner.iter_mut() {
            *i *= rhs;
        }

        self
    }
}

impl<const N: usize> std::ops::MulAssign<f32> for Vector<N> {
    fn mul_assign(&mut self, rhs: f32) {
        for i in self.inner.iter_mut() {
            *i *= rhs;
        }
    }
}

impl<const N: usize> FromIterator<f32> for Vector<N> {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        let mut vector = Self::zeroed();

        for (u, v) in vector.inner.iter_mut().zip(iter) {
            *u = v;
        }

        vector
    }
}
