use mentor::{Game, GameState};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct TicTacToe {
    pub side_to_move: bool,
    pub grid: [u16; 2],
}

#[derive(Clone, Copy)]
pub struct Move(pub u16);

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mov = self.0.trailing_zeros();
        write!(f, "{}{}", mov % 3, mov / 3)
    }
}

impl From<u16> for Move {
    fn from(mov: u16) -> Self {
        Move(mov)
    }
}

impl From<Move> for u16 {
    fn from(mov: Move) -> Self {
        mov.0
    }
}

#[allow(dead_code)]
impl TicTacToe {
    pub fn print(&self) {
        let mut grid = vec!["."; 9];

        for i in 0..9 {
            if ((self.grid[0] >> i) & 1) != 0 {
                grid.insert(i, "X");
            }

            if ((self.grid[1] >> i) & 1) != 0 {
                grid.insert(i, "O");
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
    type Move = Move;

    fn side_to_move(&self) -> usize {
        usize::from(self.side_to_move)
    }

    fn game_state(&self) -> GameState {
        const BITBOARDS: [u16; 8] = [
            0b111_000_000,
            0b000_111_000,
            0b000_000_111,
            0b100_100_100,
            0b010_010_010,
            0b001_001_001,
            0b100_010_001,
            0b001_010_100,
        ];

        let side_to_move = self.side_to_move();
        for bitboard in BITBOARDS {
            if (self.grid[side_to_move ^ 1] & bitboard).count_ones() == 3 {
                return GameState::Loss;
            }

            if (self.grid[side_to_move] & bitboard).count_ones() == 3 {
                return GameState::Win;
            }
        }

        if self.grid[0] + self.grid[1] == 511 {
            return GameState::Draw;
        }

        GameState::Ongoing
    }

    fn hash(&self) -> u64 {
        ((self.grid[0] as u64) << 9) | self.grid[1] as u64
    }

    fn get_value(&mut self) -> f32 {
        let mut pos = *self;

        let side_to_move = pos.side_to_move();

        while pos.game_state() == GameState::Ongoing {
            let moves = pos.get_legal_moves();
            let index = (rand::random::<f32>() * moves.len() as f32).floor() as usize;

            pos.make_move(moves[index]);
        }

        match pos.game_state() {
            GameState::Draw => 0.5,
            _ if side_to_move == pos.side_to_move() => 0.0,
            _ => 1.0,
        }
    }

    fn make_move(&mut self, mov: Self::Move) {
        self.grid[self.side_to_move()] |= mov.0;
        self.side_to_move ^= true;
    }

    fn get_legal_moves(&self) -> Vec<Self::Move> {
        let mut moves = Vec::new();

        if self.game_state() != GameState::Ongoing {
            return moves;
        }

        let mut index = 1u16;
        while index <= 256 {
            if ((self.grid[0] | self.grid[1]) & index) == 0 {
                moves.push(Move(index))
            }

            index <<= 1;
        }

        moves
    }
}
