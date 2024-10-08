use super::types::bitboard::Bitboard;

#[macro_export]
macro_rules! bitboard_loop {
    ($bitboard:expr, $square:ident, $func:expr) => {
        while $bitboard != Bitboard::ZERO {
            let $square = $bitboard.trailing_zeros();
            $bitboard &= $bitboard - Bitboard(1);
            $func;
        }
    };
}

macro_rules! c_enum {
    ($name:ident { $($(#[$meta:meta])? $key:ident: $type:ty = $value:expr,)+ }) => {
        pub struct $name;

        impl $name {
            $(
                $(#[$meta])?
                pub const $key: $type = $value;
            )+
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
        [Bitboard(0b0000_1110), Bitboard(0b0110_0000)],
        [Bitboard(0b0000_1110 << 56), Bitboard(0b0110_0000 << 56)]
    ],
    #[allow(dead_code)]
    ROOK_MOVES: [[(usize, usize); 2]; 2] = [[(0, 3), (7, 5)], [(56, 59), (63, 61)]],
    #[allow(dead_code)]
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
