use crate::type_enum;

use super::{bitboard::Bitboard, file::File, rank::Rank};

#[rustfmt::skip]
type_enum!(Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
});

impl Square {
    pub const fn bitboard(self) -> Bitboard {
        Bitboard(1 << (self as u64))
    }

    pub const fn file(self) -> File {
        File::ALL[(self as usize) & 7]
    }

    pub const fn rank(self) -> Rank {
        Rank::ALL[(self as usize) >> 3]
    }

    pub fn shift<const SHIFT: usize>(self, side_to_move: bool) -> Self {
        if side_to_move {
            Square::ALL[self as usize - SHIFT]
        } else {
            Square::ALL[self as usize + SHIFT]
        }
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.rank(), self.file())
    }
}
