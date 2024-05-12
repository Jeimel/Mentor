#[macro_export]
macro_rules! bitboard_loop {
    ($bitboard:expr, $square:ident, $func:expr) => {
        while $bitboard != 0 {
            let $square = $bitboard.trailing_zeros() as u8;
            $bitboard &= $bitboard - 1;
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

c_enum!(Attacks {
    NOT_FILE_A: u64 = 0xfefefefefefefefe,
    NOT_FILE_H: u64 = 0x7f7f7f7f7f7f7f7f,
    PROMO_SQUARE: [u64; 2] = [0x00FF000000000000, 0x000000000000FF00],
    END_SQUARE: [u64; 2] = [0xFF00000000000000, 0x00000000000000FF],
    KING: [u64; 64] = lookup_table!(square, 64, {
        let mut n = 1 << square;

        n |= (n << 8) | (n >> 8);
        n |= ((n & Attacks::NOT_FILE_A) >> 1) | ((n & Attacks::NOT_FILE_H) << 1);
        n ^ (1 << square)
    }),
    PAWN: [[u64; 64]; 2] = [
        lookup_table!(square, 64, {
            let n = 1 << square;

            ((n & Attacks::NOT_FILE_A) << 7) | ((n & Attacks::NOT_FILE_H) << 9)
        }),
        lookup_table!(square, 64, {
            let n = 1 << square;

            ((n & Attacks::NOT_FILE_A) >> 9) | ((n & Attacks::NOT_FILE_H) >> 7)
        }),
    ],
    KNIGHT: [u64; 64] = lookup_table!(square, 64, {
        let n = 1 << square;

        let h1 = (n >> 1) & Attacks::NOT_FILE_H | (n << 1) & Attacks::NOT_FILE_A;
        let h2 = (n >> 2) & 0x3f3f3f3f3f3f3f3f | (n << 2) & 0xfcfcfcfcfcfcfcfc;

        (h1 << 16) | (h1 >> 16) | (h2 << 8) | (h2 >> 8)
    }),
});
