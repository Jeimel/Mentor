use crate::tree::node::Node;

#[derive(Copy, Clone)]
pub struct MctsParameter {
    pub cpuct_init: f32,
    pub cpuct_base: f32,
}

impl Default for MctsParameter {
    fn default() -> Self {
        MctsParameter {
            cpuct_init: 1.41,
            cpuct_base: 1.0,
        }
    }
}

impl MctsParameter {
    pub fn new(cpuct_init: f32, cpuct_base: f32) -> Self {
        MctsParameter {
            cpuct_init,
            cpuct_base,
        }
    }

    pub fn cpuct(&self, parent: &Node) -> f32 {
        let mut cpuct = self.cpuct_init;
        cpuct += ((parent.visits() + self.cpuct_base + 1.0) / self.cpuct_base).ln();

        cpuct
    }
}

#[derive(Copy, Clone)]
pub struct SearchSettings {
    pub max_time: Option<u128>,
    pub max_nodes: usize,
}
