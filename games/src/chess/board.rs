use super::{moves::Move, types::bitboard::Bitboard};
use crate::{
    bitboard_loop,
    chess::{
        moves::{get_king_moves, get_knight_moves, get_pawn_attacks},
        util::{Attacks, Flag, Piece},
    },
};

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct Board {
    pub side_to_move: bool,
    bitboards: [Bitboard; 8],
    half_moves: u8,
    castle_rights: u8,
    en_passant_square: u8,
    check: bool,
    hash: u64,
}

impl Board {
    pub fn gen_moves(&self) -> Vec<Move> {
        let mut moves = Vec::with_capacity(35);

        let side = usize::from(self.side_to_move);
        let occupancy = self.bitboards[0] | self.bitboards[1];

        let mut pawn_mask = self.bitboards[Piece::PAWN] & self.bitboards[side];
        bitboard_loop!(pawn_mask, from, {
            let attack_mask = get_pawn_attacks(from, side);

            let shift: u64 = if self.side_to_move { 16 } else { 40 };

            let mut capture_mask =
                attack_mask & self.bitboards[side ^ 1] & !Attacks::END_SQUARE[side];
            let mut en_passant_mask =
                attack_mask & Bitboard((u64::from(self.en_passant_square)) << shift);
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
                    Piece::KNIGHT => get_knight_moves(from),
                    Piece::BISHOP => Bitboard(0), // Attacks::bishop(from as usize, occupancy),
                    Piece::ROOK => Bitboard(0),   // Attacks::rook(from as usize, occupancy),
                    Piece::QUEEN => {
                        Bitboard(0) // Attacks::bishop(from as usize, occupancy) | Attacks::rook(from as usize, occupancy)
                    }
                    Piece::KING => get_king_moves(from),
                    _ => Bitboard(0),
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

        let empty_shifted = (!occupancy).shift(self.side_to_move);
        let pawn_mask = self.bitboards[Piece::PAWN] & self.bitboards[side] & empty_shifted;

        let mut single_pawn_mask = pawn_mask & !Attacks::PROMO_SQUARE[side];
        bitboard_loop!(single_pawn_mask, from, {
            moves.push(Move {
                from,
                to: from.shift::<8>(self.side_to_move),
                flag: Flag::QUIET,
            });
        });

        let mut double_pawn_mask =
            pawn_mask & Attacks::PROMO_SQUARE[side ^ 1] & empty_shifted.shift(self.side_to_move);
        bitboard_loop!(double_pawn_mask, from, {
            moves.push(Move {
                from,
                to: from.shift::<16>(self.side_to_move),
                flag: Flag::QUIET,
            });
        });

        let mut promo_mask = pawn_mask & Attacks::PROMO_SQUARE[side];
        bitboard_loop!(promo_mask, from, {
            for piece in Piece::KNIGHT..=Piece::QUEEN {
                moves.push(Move {
                    from,
                    to: from.shift::<8>(self.side_to_move),
                    flag: (piece as u8 - 3) | Flag::PROMOTION,
                });
            }
        });

        moves
    }
}
