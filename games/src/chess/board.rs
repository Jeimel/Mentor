use super::{moves::Move, types::bitboard::Bitboard};
use crate::{
    bitboard_loop,
    chess::{
        moves::{
            get_bishop_moves, get_king_moves, get_knight_moves, get_pawn_attacks, get_queen_moves,
            get_rook_moves,
        },
        types::{rank::Rank, square::Square},
        util::{Castle, Flag, Piece},
    },
};

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct Board {
    pub side_to_move: bool,
    bitboards: [Bitboard; 8],
    half_moves: u8,
    castle_rights: u8,
    en_passant_rank: u8,
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
                attack_mask & self.bitboards[side ^ 1] & !Rank::END[side].bitboard();
            bitboard_loop!(
                capture_mask,
                to,
                moves.push(Move {
                    from,
                    to,
                    flag: Flag::CAPTURE
                })
            );

            let mut en_passant_mask =
                attack_mask & Bitboard((u64::from(self.en_passant_rank)) << shift);

            let mut promo_mask =
                attack_mask & self.bitboards[side ^ 1] & Rank::END[side].bitboard();
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
                    Piece::BISHOP => get_bishop_moves(from, occupancy),
                    Piece::ROOK => get_rook_moves(from, occupancy),
                    Piece::QUEEN => get_queen_moves(from, occupancy),
                    Piece::KING => get_king_moves(from),
                    _ => Bitboard::ZERO,
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

        let shifted_empty_squares = (!occupancy).shift(self.side_to_move);
        let pawn_mask = self.bitboards[Piece::PAWN] & self.bitboards[side] & shifted_empty_squares;

        let mut single_pawn_mask = pawn_mask & !Rank::PROMO[side].bitboard();
        bitboard_loop!(single_pawn_mask, from, {
            moves.push(Move {
                from,
                to: from.shift::<8>(self.side_to_move),
                flag: Flag::QUIET,
            });
        });

        let mut double_pawn_mask = pawn_mask
            & Rank::PROMO[side ^ 1].bitboard()
            & shifted_empty_squares.shift(self.side_to_move);
        bitboard_loop!(double_pawn_mask, from, {
            moves.push(Move {
                from,
                to: from.shift::<16>(self.side_to_move),
                flag: Flag::DOUBLE_PAWN,
            });
        });

        let mut promo_mask = pawn_mask & Rank::PROMO[side].bitboard();
        bitboard_loop!(promo_mask, from, {
            for piece in Piece::KNIGHT..=Piece::QUEEN {
                moves.push(Move {
                    from,
                    to: from.shift::<8>(self.side_to_move),
                    flag: (piece as u8 - 3) | Flag::PROMOTION,
                });
            }
        });

        if self.in_check(side) || self.castle_rights & Castle::RIGHTS[side] == 0 {
            return moves;
        }

        let (king, rook, from, to) = if self.side_to_move {
            (Castle::BLACK_KING, Square::F8, Square::E8, Square::G8)
        } else {
            (Castle::WHITE_KING, Square::F1, Square::E1, Square::G1)
        };

        if occupancy & Castle::MASK[side][0] == Bitboard::ZERO
            && self.castle_rights & king != 0
            && !self.square_attacked(rook, side, occupancy)
        {
            moves.push(Move {
                from,
                to,
                flag: Flag::KING_CASTLE,
            });
        }

        let (king, rook, to) = if self.side_to_move {
            (Castle::BLACK_QUEEN, Square::D8, Square::C8)
        } else {
            (Castle::WHITE_QUEEN, Square::D1, Square::C1)
        };

        if occupancy & Castle::MASK[side][1] == Bitboard::ZERO
            && self.castle_rights & king != 0
            && !self.square_attacked(rook, side, occupancy)
        {
            moves.push(Move {
                from,
                to,
                flag: Flag::QUEEN_CASTLE,
            });
        }

        moves
    }

    fn in_check(&self, side: usize) -> bool {
        self.square_attacked(
            (self.bitboards[Piece::KING] & self.bitboards[side]).trailing_zeros() as Square,
            side,
            self.bitboards[0] | self.bitboards[1],
        )
    }

    fn square_attacked(&self, square: Square, side: usize, occupancy: Bitboard) -> bool {
        (self.bitboards[Piece::KNIGHT] & get_knight_moves(square)
            | self.bitboards[Piece::KING] & get_king_moves(square)
            | self.bitboards[Piece::PAWN] & get_pawn_attacks(square, side)
            | self.bitboards[Piece::ROOK] & get_rook_moves(square, occupancy)
            | self.bitboards[Piece::BISHOP] & get_bishop_moves(square, occupancy)
            | self.bitboards[Piece::QUEEN]
                & (get_bishop_moves(square, occupancy) | get_rook_moves(square, occupancy)))
            & self.bitboards[side ^ 1]
            != Bitboard::ZERO
    }
}
