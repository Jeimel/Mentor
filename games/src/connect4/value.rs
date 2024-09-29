#![allow(dead_code)]

use mentor::network::layer::DenseLayer;

use super::board::Board;

struct ValueNetwork {
    l1: DenseLayer<f32, 84, 128>,
    l2: DenseLayer<f32, 128, 64>,
    l4: DenseLayer<f32, 64, 1>,
}

impl ValueNetwork {
    pub fn value(&self, _: Board) -> f32 {
        todo!()
    }
}
