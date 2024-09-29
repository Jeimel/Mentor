#[derive(Copy, Clone)]
pub struct SearchSettings {
    pub max_time: Option<u128>,
    pub max_nodes: usize,
}
