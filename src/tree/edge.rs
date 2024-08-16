#[derive(Clone, Debug)]
pub struct Edge {
    ptr: i32,
    mov: u16,
    policy: u16,
}

impl Edge {
    pub fn new(mov: u16) -> Self {
        Edge {
            ptr: -1,
            mov,
            policy: 0,
        }
    }

    pub fn ptr(&self) -> i32 {
        self.ptr
    }

    pub fn mov(&self) -> u16 {
        self.mov
    }

    pub fn policy(&self) -> f32 {
        f32::from(self.policy) / f32::from(u16::MAX)
    }

    pub fn set_ptr(&mut self, ptr: i32) {
        self.ptr = ptr;
    }

    pub fn set_policy(&mut self, policy: f32) {
        self.policy = (policy * f32::from(u16::MAX)) as u16;
    }
}
