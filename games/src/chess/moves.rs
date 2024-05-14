use crate::chess::util::Flag;
use crate::{chess::util::Attacks, lookup_table};

use super::types::bitboard::Bitboard;
use super::types::square::Square;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub flag: u8,
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const PIECES_NAME: [char; 4] = ['n', 'b', 'q', 'k'];

        let mut promo = String::from("");
        if self.flag & Flag::PROMOTION != 0 {
            promo.push(PIECES_NAME[(self.flag & 0b11) as usize])
        }

        write!(f, "{}{}{}", self.from, self.to, promo)
    }
}

impl From<u16> for Move {
    fn from(mov: u16) -> Self {
        Move {
            from: Square::ALL[(mov & 0x3F) as usize],
            to: Square::ALL[(mov & 0xFC0) as usize],
            flag: (mov & 0xFC00) as u8,
        }
    }
}

impl From<Move> for u16 {
    fn from(mov: Move) -> Self {
        (mov.from as u32 | (mov.to as u32) << 6 | (mov.flag as u32) << 12) as u16
    }
}

#[inline(always)]
pub const fn get_king_moves(square: Square) -> Bitboard {
    const KING: [Bitboard; 64] = lookup_table!(square, 64, {
        let mut n = 1 << square;

        n |= (n << 8) | (n >> 8);
        n |= ((n & Attacks::NOT_FILE_A) >> 1) | ((n & Attacks::NOT_FILE_H) << 1);
        Bitboard(n ^ 1 << square)
    });

    KING[square as usize]
}

#[inline(always)]
pub const fn get_knight_moves(square: Square) -> Bitboard {
    const KNIGHT: [Bitboard; 64] = lookup_table!(square, 64, {
        let n = 1 << square;

        let h1 = (n >> 1) & Attacks::NOT_FILE_H | (n << 1) & Attacks::NOT_FILE_A;
        let h2 = (n >> 2) & 0x3f3f3f3f3f3f3f3f | (n << 2) & 0xfcfcfcfcfcfcfcfc;

        Bitboard((h1 << 16) | (h1 >> 16) | (h2 << 8) | (h2 >> 8))
    });

    KNIGHT[square as usize]
}

#[inline(always)]
pub const fn get_pawn_attacks(square: Square, side: usize) -> Bitboard {
    const PAWN: [[Bitboard; 64]; 2] = [
        lookup_table!(square, 64, {
            let n = 1 << square;

            Bitboard(((n & Attacks::NOT_FILE_A) << 7) | ((n & Attacks::NOT_FILE_H) << 9))
        }),
        lookup_table!(square, 64, {
            let n = 1 << square;

            Bitboard(((n & Attacks::NOT_FILE_A) >> 9) | ((n & Attacks::NOT_FILE_H) >> 7))
        }),
    ];

    PAWN[side][square as usize]
}

#[inline(always)]
pub fn get_rook_moves(square: Square, occupancy: Bitboard) -> Bitboard {
    Bitboard(0)
}

#[inline(always)]
pub fn get_bishop_moves(square: Square, occupancy: Bitboard) -> Bitboard {
    Bitboard(0)
}
