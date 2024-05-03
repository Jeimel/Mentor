pub mod dense;

pub trait FeedForwardLayer: Sized {
    type InputType;
    type OutputType;

    fn sum(&self, input: &Self::InputType) -> Self::OutputType;

    fn feedforward(&self, input: &Self::InputType) -> Self::OutputType;

    fn update(&mut self, gradient: &Self, learn_rate: f32);

    fn backprop(
        &self,
        gradient: &mut Self,
        output: &Self::OutputType,
        error: Self::OutputType,
        input: &Self::InputType,
    ) -> Self::InputType;
}

pub mod activation {
    pub struct Sigmoid;
    pub struct ReLu;
    pub struct TanH;

    pub trait Activation {
        fn activate(x: f32) -> f32;

        fn derive(x: f32) -> f32;
    }

    impl Activation for Sigmoid {
        fn activate(x: f32) -> f32 {
            1.0 / (1.0 + (-x).exp())
        }

        fn derive(x: f32) -> f32 {
            let a = Sigmoid::activate(x);

            a * (1.0 - a)
        }
    }

    impl Activation for ReLu {
        fn activate(x: f32) -> f32 {
            x.max(0.0)
        }

        fn derive(x: f32) -> f32 {
            f32::from(x.is_sign_positive())
        }
    }

    impl Activation for TanH {
        fn activate(x: f32) -> f32 {
            x.tanh()
        }

        fn derive(x: f32) -> f32 {
            let a: f32 = x.tanh();

            1.0 - a * a
        }
    }
}

pub mod cost {
    use crate::collections::vector::Vector;

    pub struct MeanSquareError;

    pub trait Cost<const N: usize> {
        fn error(x: Vector<N>, y: Vector<N>) -> f32;

        fn derive(x: f32, y: f32) -> f32;
    }

    impl<const N: usize> Cost<N> for MeanSquareError {
        fn error(x: Vector<N>, y: Vector<N>) -> f32 {
            let mut error = 0.0;

            for i in 0..N {
                error += (x[i] - y[i]) * (x[i] - y[i])
            }

            0.5 * error
        }

        fn derive(x: f32, y: f32) -> f32 {
            x - y
        }
    }
}
