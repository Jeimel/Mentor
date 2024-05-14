use std::ops::{Add, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, Div, Mul, Sub};

use super::square::Square;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub fn trailing_zeros(&self) -> Square {
        Square::ALL[self.0.trailing_zeros() as usize]
    }

    pub fn shift(&self, side_to_move: bool) -> Self {
        if side_to_move {
            Bitboard(self.0 << 8)
        } else {
            Bitboard(self.0 >> 8)
        }
    }
}

impl std::ops::Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self {
        Bitboard(!self.0)
    }
}

macro_rules! impl_math_ops {
    ($($trait:ident, $fn:ident $(,$trait_assign:ident, $fn_assign:ident)?;)*) => {$(
        impl $trait for Bitboard {
            type Output = Self;

            #[inline(always)]
            fn $fn(self, rhs: Self) -> Self::Output {
                Self($trait::$fn(self.0, rhs.0))
            }
        }

        $(
            impl $trait_assign for Bitboard {
                #[inline(always)]
                fn $fn_assign(&mut self, rhs: Self) {
                    self.0 = $trait::$fn(self.0, rhs.0);
                }
            }
        )?
    )*};
}

impl_math_ops!(
    Add, add;
    Sub, sub;
    Mul, mul;
    Div, div;
    BitAnd, bitand, BitAndAssign, bitand_assign;
    BitOr, bitor, BitOrAssign, bitor_assign;
    BitXor, bitxor;
);
