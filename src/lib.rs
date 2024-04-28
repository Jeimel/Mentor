pub mod mcts;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum GameState {
    #[default]
    Ongoing,
    Win,
    Draw,
    Loss,
}

pub trait Game: Clone + Copy + Default {
    type Move: std::fmt::Display + From<u16> + Into<u16> + Copy;

    fn game_state(&self) -> GameState;

    fn make_move(&mut self, mov: Self::Move);

    fn get_moves(&self) -> Vec<Self::Move>;
}

#[derive(Clone, Copy)]
pub struct TicTacToe {
    pub turn: usize,
    pub grid: [u16; 2],
}

#[derive(Clone, Copy)]
pub struct TicTacToeMove(pub u16);

impl Default for TicTacToe {
    fn default() -> Self {
        Self {
            turn: 0,
            grid: [0, 0],
        }
    }
}

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

impl Into<u16> for TicTacToeMove {
    fn into(self) -> u16 {
        self.0
    }
}

impl Game for TicTacToe {
    type Move = TicTacToeMove;

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

        if self.grid[0] + self.grid[1] == 511 {
            return GameState::Draw;
        }

        for bitboard in bitboards {
            if (self.grid[0] & bitboard).count_ones() == 3 {
                return GameState::Win;
            }

            if (self.grid[1] & bitboard).count_ones() == 3 {
                return GameState::Loss;
            }
        }

        return GameState::Ongoing;
    }

    fn make_move(&mut self, mov: Self::Move) {
        self.grid[self.turn] |= mov.0;
        self.turn = (self.turn + 1) % 2;
    }

    fn get_moves(&self) -> Vec<Self::Move> {
        let mut moves = Vec::new();

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
