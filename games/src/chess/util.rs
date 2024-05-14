use super::types::bitboard::Bitboard;

#[macro_export]
macro_rules! bitboard_loop {
    ($bitboard:expr, $square:ident, $func:expr) => {
        while $bitboard != Bitboard(0) {
            let $square = $bitboard.trailing_zeros();
            $bitboard &= $bitboard - Bitboard(1);
            $func;
        }
    };
}

macro_rules! c_enum {
    ($name:ident { $($key:ident: $type:ty = $value:expr,)+ }) => {
        pub struct $name;

        impl $name {
            $(pub const $key: $type = $value;)+
        }
    };
}

#[macro_export]
macro_rules! lookup_table {
    ($square:ident, $size:expr, $($pattern:tt)+) => {
        {
            let mut $square = 0;
            let mut table = [$($pattern)+; $size];

            while $square < $size - 1 {
                $square += 1;
                table[$square] = $($pattern)+;
            }

            table
        }
    };
}

c_enum!(Piece {
    PAWN: usize = 2,
    KNIGHT: usize = 3,
    BISHOP: usize = 4,
    ROOK: usize = 5,
    QUEEN: usize = 6,
    KING: usize = 7,
});

c_enum!(Flag {
    QUIET: u8 = 0,
    DOUBLE_PAWN: u8 = 1,
    QUEEN_CASTLE: u8 = 2,
    KING_CASTLE: u8 = 3,
    CAPTURE: u8 = 4,
    EN_PASSANT: u8 = 5,
    PROMOTION: u8 = 8,
});

c_enum!(Castle {
    BLACK_KING: u8 = 1,
    BLACK_QUEEN: u8 = 2,
    WHITE_KING: u8 = 4,
    WHITE_QUEEN: u8 = 8,
    RIGHTS: [u8; 2] = [
        Castle::WHITE_QUEEN | Castle::WHITE_KING,
        Castle::BLACK_QUEEN | Castle::BLACK_KING
    ],
    MASK: [[Bitboard; 2]; 2] = [
        [Bitboard(0b00001110), Bitboard(0b01100000)],
        [Bitboard(0b00001110 << 56), Bitboard(0b01100000 << 56)]
    ],
    ROOK_MOVES: [[(usize, usize); 2]; 2] = [[(0, 3), (7, 5)], [(56, 59), (63, 61)]],
    MOVES: [u8; 64] = lookup_table!(square, 64, {
        match square {
            0 => 7,
            4 => 3,
            7 => 11,
            56 => 13,
            60 => 12,
            63 => 14,
            _ => 15,
        }
    }),
});

c_enum!(Attacks {
    NOT_FILE_A: u64 = 0xfefefefefefefefe,
    NOT_FILE_H: u64 = 0x7f7f7f7f7f7f7f7f,
    PROMO_RANK: [Bitboard; 2] = [Bitboard(0x00FF000000000000), Bitboard(0x000000000000FF00)],
    END_RANK: [Bitboard; 2] = [Bitboard(0xFF00000000000000), Bitboard(0x00000000000000FF)],
});
