use crate::tree::node::Node;

#[derive(Copy, Clone)]
pub struct SearchParameter {
    pub cpuct_init: f32,
    pub cpuct_base: f32,
}

impl Default for SearchParameter {
    fn default() -> Self {
        SearchParameter {
            cpuct_init: 1.41,
            cpuct_base: 1.0,
        }
    }
}

impl SearchParameter {
    pub fn new(cpuct_init: f32, cpuct_base: f32) -> Self {
        SearchParameter {
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
