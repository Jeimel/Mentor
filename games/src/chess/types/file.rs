use crate::type_enum;

use super::bitboard::Bitboard;

#[rustfmt::skip]
type_enum!(File {
    A, B, C, D, E, F, G, H, 
});

impl File {
    #[inline]
    pub const fn bitboard(self) -> Bitboard {
        const FILE: [Bitboard; 8] = [
            Bitboard(0x0101010101010101),
            Bitboard(0x0202020202020202),
            Bitboard(0x0404040404040404),
            Bitboard(0x0808080808080808),
            Bitboard(0x1010101010101010),
            Bitboard(0x2020202020202020),
            Bitboard(0x4040404040404040),
            Bitboard(0x8080808080808080),
        ];

        FILE[self as usize]
    }
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(97u8 - ((*self as u8) & 7)))
    }
}
