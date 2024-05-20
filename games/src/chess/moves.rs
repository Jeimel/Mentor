use crate::chess::types::bitboard::Bitboard;
use crate::chess::types::file::File;
use crate::chess::util::{Flag, DIAGONALS};
use crate::lookup_table;

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
            from: Square::ALL[(mov & 0b0000_000000_111111) as usize],
            to: Square::ALL[(mov & 0b0000_111111_000000) as usize],
            flag: (mov & 0b1111_000000_000000) as u8,
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
        n |= ((n & !File::A.bitboard().0) >> 1) | ((n & !File::H.bitboard().0) << 1);
        Bitboard(n ^ 1 << square)
    });

    KING[square as usize]
}

#[inline(always)]
pub const fn get_knight_moves(square: Square) -> Bitboard {
    const KNIGHT: [Bitboard; 64] = lookup_table!(square, 64, {
        let n = 1 << square;

        let h1 = (n >> 1) & !File::H.bitboard().0 | (n << 1) & !File::A.bitboard().0;
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

            Bitboard(((n & !File::A.bitboard().0) << 7) | ((n & !File::H.bitboard().0) << 9))
        }),
        lookup_table!(square, 64, {
            let n = 1 << square;

            Bitboard(((n & !File::A.bitboard().0) >> 9) | ((n & !File::H.bitboard().0) >> 7))
        }),
    ];

    PAWN[side][square as usize]
}

#[inline(always)]
pub fn get_rook_moves(square: Square, occupancy: Bitboard) -> Bitboard {
    Bitboard::ZERO
}

#[inline(always)]
pub fn get_bishop_moves(square: Square, occupancy: Bitboard) -> Bitboard {
    const BISHOP: [Mask; 64] = lookup_table!(square, 64, {
        let n = 1 << square;

        let file = square & 7;
        let rank = square >> 3;

        Mask {
            square: Bitboard(n),
            square_swapped: Bitboard(n.swap_bytes()),
            diagonal: Bitboard(n ^ DIAGONALS[7 + file - rank]),
            anti_diagonal: Bitboard(n ^ DIAGONALS[file + rank].swap_bytes()),
        }
    });

    let mask = BISHOP[square as usize];

    let mut diag = occupancy & mask.diagonal;
    let mut rev1 = diag.swap_bytes();
    diag = diag.wrapping_sub(mask.square);
    rev1 = rev1.wrapping_sub(mask.square_swapped);
    diag ^= rev1.swap_bytes();
    diag &= mask.diagonal;

    let mut anti = occupancy & mask.anti_diagonal;
    let mut rev2 = anti.swap_bytes();
    anti = anti.wrapping_sub(mask.square);
    rev2 = rev2.wrapping_sub(mask.square_swapped);
    anti ^= rev2.swap_bytes();
    anti &= mask.anti_diagonal;

    diag | anti
}

#[derive(Clone, Copy)]
struct Mask {
    square: Bitboard,
    square_swapped: Bitboard,
    diagonal: Bitboard,
    anti_diagonal: Bitboard,
}
