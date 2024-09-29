#![allow(dead_code)]

pub struct Sigmoid;
pub struct ReLu;
pub struct TanH;

pub trait Activation {
    fn activate(x: f32) -> f32;
}

impl Activation for Sigmoid {
    fn activate(x: f32) -> f32 {
        1.0 / (1.0 + (-x).exp())
    }
}

impl Activation for ReLu {
    fn activate(x: f32) -> f32 {
        x.max(0.0)
    }
}

impl Activation for TanH {
    fn activate(x: f32) -> f32 {
        x.tanh()
    }
}
