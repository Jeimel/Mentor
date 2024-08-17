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
            Bitboard(0x0101_0101_0101_0101),
            Bitboard(0x0202_0202_0202_0202),
            Bitboard(0x0404_0404_0404_0404),
            Bitboard(0x0808_0808_0808_0808),
            Bitboard(0x1010_1010_1010_1010),
            Bitboard(0x2020_2020_2020_2020),
            Bitboard(0x4040_4040_4040_4040),
            Bitboard(0x8080_8080_8080_8080),
        ];

        FILE[self as usize]
    }
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(97u8 - ((*self as u8) & 7)))
    }
}
