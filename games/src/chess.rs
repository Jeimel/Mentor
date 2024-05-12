mod util;

use crate::{
    bitboard_loop,
    chess::util::{Attacks, Flag},
};

use self::util::Piece;
use mentor::Game;

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct Chess {
    side_to_move: bool,
    bitboards: [u64; 8],
    pub en_passant_square: u8,
    pub half_moves: u8,
    pub castle_rights: u8,
    pub check: bool,
    pub hash: u64,
}

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub flag: u8,
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        pub const PIECES_NAME: [char; 4] = ['n', 'b', 'q', 'k'];

        let mut promo = String::from("");
        if self.flag & Flag::PROMOTION != 0 {
            promo.push(PIECES_NAME[(self.flag & 0b11) as usize])
        }

        let from_square = |square| {
            format!(
                "{}{}",
                char::from(97u8 - (square & 0b111)),
                (square >> 3) + 1u8
            )
        };

        write!(
            f,
            "{}{}{}",
            from_square(self.from),
            from_square(self.to),
            promo
        )
    }
}

impl From<u16> for Move {
    fn from(mov: u16) -> Self {
        Move {
            from: (mov & 0x3F) as u8,
            to: (mov & 0xFC0) as u8,
            flag: (mov & 0xFC00) as u8,
        }
    }
}

impl From<Move> for u16 {
    fn from(mov: Move) -> Self {
        ((mov.from as u32) | (mov.to as u32) << 6 | (mov.flag as u32) << 12) as u16
    }
}

impl Game for Chess {
    type Move = Move;

    fn side_to_move(&self) -> usize {
        usize::from(self.side_to_move)
    }

    fn game_state(&self) -> mentor::GameState {
        todo!()
    }

    fn hash(&self) -> u64 {
        todo!()
    }

    fn get_value(&mut self) -> f32 {
        todo!()
    }

    fn make_move(&mut self, mov: Self::Move) {
        todo!()
    }

    fn get_legal_moves(&self) -> Vec<Self::Move> {
        let mut moves = Vec::with_capacity(35);

        let side = usize::from(self.side_to_move);
        let occupancy = self.bitboards[0] | self.bitboards[1];

        let mut pawn_mask = self.bitboards[Piece::PAWN] & self.bitboards[side];
        bitboard_loop!(pawn_mask, from, {
            let attack_mask = Attacks::PAWN[side][from as usize];

            let shift: u64 = if self.side_to_move { 16 } else { 40 };

            let mut capture_mask =
                attack_mask & self.bitboards[side ^ 1] & !Attacks::END_SQUARE[side];
            let mut en_passant_mask = attack_mask & ((u64::from(self.en_passant_square)) << shift);
            let mut promo_mask = attack_mask & self.bitboards[side ^ 1] & Attacks::END_SQUARE[side];

            bitboard_loop!(
                capture_mask,
                to,
                moves.push(Move {
                    from,
                    to,
                    flag: Flag::CAPTURE
                })
            );
            bitboard_loop!(
                en_passant_mask,
                to,
                moves.push(Move {
                    from,
                    to,
                    flag: Flag::EN_PASSANT
                })
            );
            bitboard_loop!(promo_mask, to, {
                for piece in Piece::KNIGHT..=Piece::QUEEN {
                    moves.push(Move {
                        from,
                        to,
                        flag: (piece as u8 - 3) | Flag::CAPTURE | Flag::PROMOTION,
                    });
                }
            });
        });

        for piece in Piece::KNIGHT..=Piece::KING {
            let mut piece_mask = self.bitboards[piece] & self.bitboards[side];

            bitboard_loop!(piece_mask, from, {
                let attack_mask = match piece {
                    Piece::KNIGHT => Attacks::KNIGHT[from as usize],
                    Piece::BISHOP => 0, // Attacks::bishop(from as usize, occupancy),
                    Piece::ROOK => 0,   // Attacks::rook(from as usize, occupancy),
                    Piece::QUEEN => {
                        0 // Attacks::bishop(from as usize, occupancy) | Attacks::rook(from as usize, occupancy)
                    }
                    Piece::KING => Attacks::KING[from as usize],
                    _ => 0,
                };

                let mut captures = attack_mask & self.bitboards[side ^ 1];
                let mut quiets = attack_mask & !occupancy;

                bitboard_loop!(
                    captures,
                    to,
                    moves.push(Move {
                        from,
                        to,
                        flag: Flag::CAPTURE
                    })
                );
                bitboard_loop!(
                    quiets,
                    to,
                    moves.push(Move {
                        from,
                        to,
                        flag: Flag::QUIET
                    })
                );
            });
        }

        // castling
        // pawn pushes (single/double)

        moves
    }
}
