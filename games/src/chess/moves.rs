use crate::chess::types::bitboard::Bitboard;
use crate::chess::types::file::File;
use crate::chess::util::Flag;
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
    #[derive(Clone, Copy)]
    struct RookMask {
        square: Bitboard,
        square_swapped: Bitboard,
        file: Bitboard,
    }

    const FILE: [RookMask; 64] = lookup_table!(square, 64, {
        let n = 1 << square;

        let file = square & 7;

        RookMask {
            square: Bitboard(n),
            square_swapped: Bitboard(n.swap_bytes()),
            file: Bitboard(n ^ File::ALL[file].bitboard().0),
        }
    });

    const RANK: [Bitboard; 512] = lookup_table!(square, 512, {
        let file = (square as u64) & 7;
        let occupancy = (square >> 2) & 2 * 63;

        let mut attacks = Bitboard(0);

        let mut i = (file).wrapping_sub(1);
        while i != u64::MAX {
            attacks.0 |= 1 << i;
            if occupancy & (1 << i) != 0 {
                break;
            }

            i = i.wrapping_sub(1);
        }

        let mut i = file + 1;
        while i < 8 {
            attacks.0 |= 1 << i;
            if occupancy & (1 << i) != 0 {
                break;
            }

            i += 1;
        }

        attacks
    });

    let mask = FILE[square as usize];

    let mut file = occupancy & mask.file;
    let mut reverse = file.swap_bytes();
    file = file.wrapping_sub(mask.square);
    reverse = reverse.wrapping_sub(mask.square_swapped);
    file ^= reverse.swap_bytes();
    file &= mask.file;

    let square = square as u64;
    let rank_x8 = square & 56;
    let rank_occ_x2 = (occupancy.0 >> rank_x8) & 2 * 63;
    let rank = Bitboard(RANK[(4 * rank_occ_x2 + (square & 7)) as usize].0 << rank_x8);

    file | rank
}

#[inline(always)]
pub fn get_bishop_moves(square: Square, occupancy: Bitboard) -> Bitboard {
    #[derive(Clone, Copy)]
    struct BishopMask {
        square: Bitboard,
        square_swapped: Bitboard,
        diagonal: Bitboard,
        anti_diagonal: Bitboard,
    }

    const BISHOP: [BishopMask; 64] = lookup_table!(square, 64, {
        pub const DIAGONALS: [u64; 15] = [
            0x0100000000000000,
            0x0201000000000000,
            0x0402010000000000,
            0x0804020100000000,
            0x1008040201000000,
            0x2010080402010000,
            0x4020100804020100,
            0x8040201008040201,
            0x0080402010080402,
            0x0000804020100804,
            0x0000008040201008,
            0x0000000080402010,
            0x0000000000804020,
            0x0000000000008040,
            0x0000000000000080,
        ];

        let n = 1 << square;

        let file = square & 7;
        let rank = square >> 3;

        BishopMask {
            square: Bitboard(n),
            square_swapped: Bitboard(n.swap_bytes()),
            diagonal: Bitboard(n ^ DIAGONALS[7 + file - rank]),
            anti_diagonal: Bitboard(n ^ DIAGONALS[file + rank].swap_bytes()),
        }
    });

    let mask = BISHOP[square as usize];

    let mut diagonal = occupancy & mask.diagonal;
    let mut reverse = diagonal.swap_bytes();
    diagonal = diagonal.wrapping_sub(mask.square);
    reverse = reverse.wrapping_sub(mask.square_swapped);
    diagonal ^= reverse.swap_bytes();
    diagonal &= mask.diagonal;

    let mut anti_diagonal = occupancy & mask.anti_diagonal;
    let mut reverse = anti_diagonal.swap_bytes();
    anti_diagonal = anti_diagonal.wrapping_sub(mask.square);
    reverse = reverse.wrapping_sub(mask.square_swapped);
    anti_diagonal ^= reverse.swap_bytes();
    anti_diagonal &= mask.anti_diagonal;

    diagonal | anti_diagonal
}

#[inline(always)]
pub fn get_queen_moves(square: Square, occupancy: Bitboard) -> Bitboard {
    get_bishop_moves(square, occupancy) | get_rook_moves(square, occupancy)
}
