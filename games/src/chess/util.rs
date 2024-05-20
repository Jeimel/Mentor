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

pub const DIAGONALS: [u64; 15] = [
    0x0100_0000_0000_0000,
    0x0201_0000_0000_0000,
    0x0402_0100_0000_0000,
    0x0804_0201_0000_0000,
    0x1008_0402_0100_0000,
    0x2010_0804_0201_0000,
    0x4020_1008_0402_0100,
    0x8040_2010_0804_0201,
    0x0080_4020_1008_0402,
    0x0000_8040_2010_0804,
    0x0000_0080_4020_1008,
    0x0000_0000_8040_2010,
    0x0000_0000_0080_4020,
    0x0000_0000_0000_8040,
    0x0000_0000_0000_0080,
];
