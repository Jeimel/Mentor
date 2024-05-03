use mentor::{Game, GameState};

#[derive(Clone, Copy, Default)]
pub struct TicTacToe {
    pub side_to_move: usize,
    pub grid: [u16; 2],
}

#[derive(Clone, Copy)]
pub struct TicTacToeMove(pub u16);

impl std::fmt::Display for TicTacToeMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.trailing_zeros())
    }
}

impl From<u16> for TicTacToeMove {
    fn from(mov: u16) -> Self {
        TicTacToeMove(mov)
    }
}

impl From<TicTacToeMove> for u16 {
    fn from(value: TicTacToeMove) -> Self {
        value.0
    }
}

impl TicTacToe {
    pub fn print(&self) {
        let mut grid = vec!["_"; 9];

        for i in 0..9 {
            if ((self.grid[0] >> i) & 1) != 0 {
                grid.insert(i, "X");
            }

            if ((self.grid[1] >> i) & 1) != 0 {
                grid.insert(i, "0");
            }
        }

        for (i, square) in grid.iter().enumerate().take(9) {
            print!("{}", square);

            if (i + 1) % 3 == 0 {
                println!();
            }
        }
        println!()
    }
}

impl Game for TicTacToe {
    type Move = TicTacToeMove;

    fn side_to_move(&self) -> usize {
        self.side_to_move
    }

    fn game_state(&self) -> GameState {
        let bitboards = [
            0b111_000_000,
            0b000_111_000,
            0b000_000_111,
            0b100_100_100,
            0b010_010_010,
            0b001_001_001,
            0b100_010_001,
            0b001_010_100,
        ];

        for bitboard in bitboards {
            if (self.grid[(self.side_to_move + 1) % 2] & bitboard).count_ones() == 3 {
                return GameState::Loss;
            }

            if (self.grid[self.side_to_move] & bitboard).count_ones() == 3 {
                return GameState::Win;
            }
        }

        if self.grid[0] + self.grid[1] == 511 {
            return GameState::Draw;
        }

        GameState::Ongoing
    }

    fn make_move(&mut self, mov: Self::Move) {
        if self.game_state() != GameState::Ongoing {
            panic!();
        }

        self.grid[self.side_to_move] |= mov.0;
        self.side_to_move = (self.side_to_move + 1) % 2;
    }

    fn get_moves(&self) -> Vec<Self::Move> {
        let mut moves = Vec::new();

        if self.game_state() != GameState::Ongoing {
            return moves;
        }

        let mut index = 1u16;
        while index <= 256 {
            if ((self.grid[0] | self.grid[1]) & index) == 0 {
                moves.push(TicTacToeMove(index))
            }

            index <<= 1;
        }

        moves
    }
}
