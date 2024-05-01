use std::marker::PhantomData;

use crate::network::collections::{matrix::Matrix, vector::Vector};

use super::{activation::Activation, FeedForwardLayer};

#[derive(Clone, Copy)]
pub struct DenseLayer<T: Activation, const M: usize, const N: usize> {
    weights: Matrix<M, N>,
    bias: Vector<M>,
    phantom: PhantomData<T>,
}

impl<T: Activation, const M: usize, const N: usize> FeedForwardLayer for DenseLayer<T, M, N> {
    type InputType = Vector<N>;
    type OutputType = Vector<M>;

    fn sum(&self, input: &Self::InputType) -> Self::OutputType {
        self.weights * *input + self.bias
    }

    fn feedforward(&self, input: &Self::InputType) -> Self::OutputType {
        self.sum(input).activate::<T>()
    }

    fn update(&mut self, gradient: &Self, learn_rate: f32) {
        self.weights -= gradient.weights * learn_rate;
        self.bias -= gradient.bias * learn_rate;
    }

    fn backprop(
        &self,
        gradient: &mut Self,
        output: &Self::OutputType,
        mut error: Self::OutputType,
        input: &Self::InputType,
    ) -> Self::InputType {
        error = error * output.derive::<T>();

        for (i, row) in gradient.weights.iter_mut().enumerate() {
            *row += *input * error[i];
        }

        gradient.bias += error;

        self.weights.transpose_mul(error)
    }
}

#[allow(dead_code)]
impl<T: Activation, const M: usize, const N: usize> DenseLayer<T, M, N> {
    pub fn random(normal: bool) -> Self {
        Self {
            weights: Matrix::rand(normal),
            bias: Vector::rand(normal),
            phantom: PhantomData,
        }
    }

    pub const fn zeroed() -> Self {
        Self {
            weights: Matrix::zeroed(),
            bias: Vector::zeroed(),
            phantom: PhantomData,
        }
    }

    pub const fn from_raw(weights: Matrix<M, N>, bias: Vector<M>) -> Self {
        Self {
            weights,
            bias,
            phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::network::{
        collections::{matrix::Matrix, vector::Vector},
        layer::{activation::ReLu, FeedForwardLayer},
    };

    use super::DenseLayer;

    #[test]
    fn feedforward() {
        let weights = Matrix::from_raw([
            Vector::from_raw([10.0, 2.0, 3.0]),
            Vector::from_raw([7.0, 6.0, 3.0]),
        ]);
        let bias = Vector::from_raw([3.0, 2.0]);
        let layer = DenseLayer::<ReLu, 2, 3>::from_raw(weights, bias);

        let input = Vector::from_raw([1.0, 2.5, 12.0]);
        let a = layer.feedforward(&input);

        let result = [54.0, 60.0];
        for i in 0..result.len() {
            assert_eq!(a[i], result[i]);
        }
    }

    #[test]
    fn backprop() {
        let weights = Matrix::from_raw([
            Vector::from_raw([10.0, 2.0, -3.0]),
            Vector::from_raw([7.0, -6.0, 3.0]),
        ]);
        let bias = Vector::from_raw([3.0, 2.0]);
        let layer = DenseLayer::<ReLu, 2, 3>::from_raw(weights, bias);

        let mut gradient = DenseLayer::<ReLu, 2, 3>::from_raw(Matrix::zeroed(), Vector::zeroed());
        let output = Vector::from_raw([54.0, 60.0]);
        let input = Vector::from_raw([2.0, -1.0, 5.0]);

        let error = Vector::from_raw([1.0, -2.5]);
        let error = layer.backprop(&mut gradient, &output, error, &input);

        let result = [-7.5, 17.0, -10.5];
        for i in 0..result.len() {
            assert_eq!(error[i], result[i]);
        }
    }
}
