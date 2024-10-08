#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct Board {
    side_to_move: bool,
    current: u64,
    mask: u64,
}

impl Board {
    pub const WIDTH: usize = 7;
    pub const HEIGHT: usize = 6;

    pub fn make_move(&mut self, col: u16) {
        self.current ^= self.mask;
        self.mask |= self.mask + Board::bottom_mask(col as usize);
        self.side_to_move ^= true;
    }

    pub fn get_moves(&self) -> Vec<u16> {
        let mut moves = Vec::new();

        for col in 0..Board::WIDTH {
            if (self.mask & Board::top_mask(col)) == 0 {
                moves.push(col as u16);
            }
        }

        moves
    }

    pub fn side_to_move(&self) -> bool {
        self.side_to_move
    }

    pub fn current(&self) -> u64 {
        self.current
    }

    pub fn mask(&self) -> u64 {
        self.mask
    }

    pub fn display(&self) -> String {
        let mut board = String::new();

        for row in (0..Board::HEIGHT).rev() {
            board.push_str("+---+---+---+---+---+---+---+\n");

            for col in 0..Board::WIDTH {
                let i = row + col * (Board::HEIGHT + 1);

                let cell = if ((self.current >> i) & 1) != 0 {
                    if self.side_to_move {
                        'X'
                    } else {
                        'O'
                    }
                } else if (((self.current ^ self.mask) >> i) & 1) != 0 {
                    if self.side_to_move {
                        '0'
                    } else {
                        'X'
                    }
                } else {
                    ' '
                };

                board.push_str(&format!("| {} ", cell));
            }

            board.push('|');
            board.push('\n');
        }

        board.push_str("+---+---+---+---+---+---+---+");
        board
    }

    pub fn alignment(pos: u64) -> bool {
        let mut m = pos & (pos >> (Board::HEIGHT + 1));
        if (m & (m >> (2 * (Board::HEIGHT + 1)))) > 0 {
            return true;
        }

        m = pos & (pos >> Board::HEIGHT);
        if (m & (m >> (2 * Board::HEIGHT))) > 0 {
            return true;
        }

        m = pos & (pos >> (Board::HEIGHT + 2));
        if (m & (m >> (2 * (Board::HEIGHT + 2)))) > 0 {
            return true;
        }

        m = pos & (pos >> 1);
        if (m & (m >> 2)) > 0 {
            return true;
        }

        false
    }

    fn top_mask(col: usize) -> u64 {
        (1u64 << (Board::HEIGHT - 1)) << (col * (Board::HEIGHT + 1))
    }

    fn bottom_mask(col: usize) -> u64 {
        1u64 << (col * (Board::HEIGHT + 1))
    }
}
