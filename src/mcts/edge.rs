#[derive(Clone, Debug)]
pub struct Edge {
    ptr: i32,
    mov: u16,
}

impl Edge {
    pub fn new(mov: u16) -> Self {
        Edge { ptr: -1, mov }
    }

    pub fn ptr(&self) -> i32 {
        self.ptr
    }

    pub fn mov(&self) -> u16 {
        self.mov
    }

    pub fn set_ptr(&mut self, ptr: i32) {
        self.ptr = ptr;
    }
}
