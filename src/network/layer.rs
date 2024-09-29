use super::{activation::Activation, vector::Vector};

pub trait Layer {
    type InputType: Clone + Copy;
    type OutputType: Clone + Copy;

    fn forward<T: Activation>(&self, input: Self::InputType) -> Self::OutputType;
}

pub struct DenseLayer<T, const N: usize, const M: usize> {
    weights: [Vector<T, N>; M],
    bias: Vector<T, M>,
}

impl<const N: usize, const M: usize> Layer for DenseLayer<f32, N, M> {
    type InputType = Vector<f32, N>;
    type OutputType = Vector<f32, M>;

    fn forward<T: Activation>(&self, input: Self::InputType) -> Self::OutputType {
        let mut activations = self.bias;

        for (activation, weights) in activations.0.iter_mut().zip(self.weights.iter()) {
            *activation = T::activate(weights.dot(&input) + *activation);
        }

        activations
    }
}
