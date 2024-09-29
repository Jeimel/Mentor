use std::ops::{Add, AddAssign, Mul, MulAssign};

use super::activation::Activation;

#[derive(Copy, Clone)]
pub struct Vector<T, const N: usize>(pub [T; N]);

impl<const N: usize> Vector<f32, N> {
    pub fn activate<T: Activation>(mut self) -> Self {
        for i in &mut self.0 {
            *i = T::activate(*i);
        }

        self
    }

    pub fn dot(&self, other: &Vector<f32, N>) -> f32 {
        self.0.iter().zip(other.0.iter()).map(|(a, b)| a * b).sum()
    }
}

impl<T: Copy, const N: usize> IntoIterator for Vector<T, N> {
    type Item = T;

    type IntoIter = std::array::IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T: Copy + AddAssign, const N: usize> Add for Vector<T, N> {
    type Output = Vector<T, N>;

    fn add(mut self, rhs: Vector<T, N>) -> Self::Output {
        for (i, j) in self.0.iter_mut().zip(rhs.0.iter()) {
            *i += *j;
        }

        self
    }
}

impl<T: Copy + MulAssign, const N: usize> Mul for Vector<T, N> {
    type Output = Vector<T, N>;

    fn mul(mut self, rhs: Vector<T, N>) -> Self::Output {
        for (i, j) in self.0.iter_mut().zip(rhs.0.iter()) {
            *i *= *j;
        }

        self
    }
}
