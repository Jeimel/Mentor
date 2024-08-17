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
            Bitboard(0x0000_0000_0000_00FF),
            Bitboard(0x0000_0000_0000_FF00),
            Bitboard(0x0000_0000_00FF_0000),
            Bitboard(0x0000_0000_FF00_0000),
            Bitboard(0x0000_00FF_0000_0000),
            Bitboard(0x0000_FF00_0000_0000),
            Bitboard(0x00FF_0000_0000_0000),
            Bitboard(0xFF00_0000_0000_0000),
        ];

        RANK[self as usize]
    }
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ((*self as usize) >> 3) + 1)
    }
}
