pub mod bitboard;
pub mod file;
pub mod rank;
pub mod square;

#[macro_export]
macro_rules! type_enum {
    ($name:ident {$($square:ident,)*}) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum $name {
            $($square),*
        }

        impl $name {
            pub const NUM: usize = [$(Self::$square),*].len();
            pub const ALL: [Self; Self::NUM] = [$(Self::$square),*];
        }
    };
}
