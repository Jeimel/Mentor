#[derive(Clone, Copy)]
pub struct Move(pub u16);

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.trailing_zeros())
    }
}

impl From<u16> for Move {
    fn from(mov: u16) -> Self {
        Move(mov)
    }
}

impl From<Move> for u16 {
    fn from(mov: Move) -> Self {
        mov.0
    }
}
