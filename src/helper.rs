use crate::tree::node::Node;

pub struct MctsParameter {
    cpuct_init: f32,
    cpuct_base: f32,
}

impl MctsParameter {
    #[must_use]
    pub fn new(cpuct_init: f32, cpuct_base: f32) -> Self {
        MctsParameter {
            cpuct_init,
            cpuct_base,
        }
    }

    #[must_use]
    pub fn cpuct(&self, parent: &Node) -> f32 {
        let mut cpuct = self.cpuct_init;
        cpuct += ((parent.visits() + self.cpuct_base + 1.0) / self.cpuct_base).ln();

        cpuct
    }
}

pub struct SearchSettings {
    pub max_time: Option<u128>,
    pub max_nodes: usize,
}
