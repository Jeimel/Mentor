use crate::{chess::types::bitboard::Bitboard, type_enum};

#[rustfmt::skip]
type_enum!(Rank {
    First, Second, Third, Fourth, Fifth, Sixth, Seventh, Eighth,
});

impl Rank {
    pub const PROMO: [Rank; 2] = [Rank::Seventh, Rank::Second];
    pub const END: [Rank; 2] = [Rank::Eighth, Rank::First];

    #[inline]
    pub const fn bitboard(self) -> Bitboard {
        const RANK: [Bitboard; 8] = [
            Bitboard(0x00000000000000FF),
            Bitboard(0x000000000000FF00),
            Bitboard(0x0000000000FF0000),
            Bitboard(0x00000000FF000000),
            Bitboard(0x000000FF00000000),
            Bitboard(0x0000FF0000000000),
            Bitboard(0x00FF000000000000),
            Bitboard(0xFF00000000000000),
        ];

        RANK[self as usize]
    }
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ((*self as usize) >> 3) + 1)
    }
}
