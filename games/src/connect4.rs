use mentor::{Game, GameState};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Connect4 {
    pub side_to_move: bool,
    pub current: u64,
    pub mask: u64,
}

#[derive(Clone, Copy)]
pub struct Move(pub u16);

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.trailing_zeros())
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

impl Game for Connect4 {
    type Move = Move;

    fn side_to_move(&self) -> usize {
        usize::from(self.side_to_move)
    }

    fn game_state(&self) -> GameState {
        if Connect4::alignment(self.current ^ self.mask) {
            return GameState::Loss;
        }

        if Connect4::alignment(self.current) {
            return GameState::Win;
        }

        if self.mask.count_ones() == (Connect4::WIDTH * Connect4::HEIGHT) as u32 {
            return GameState::Draw;
        }

        GameState::Ongoing
    }

    fn hash(&self) -> u64 {
        self.current + self.mask
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
            GameState::Draw => 0.0,
            _ if side_to_move == pos.side_to_move() => -1.0,
            _ => 1.0,
        }
    }

    fn make_move(&mut self, mov: Self::Move) {
        let col: u16 = mov.into();

        self.current ^= self.mask;
        self.mask |= self.mask + Connect4::bottom_mask(col as usize);
        self.side_to_move ^= true;
    }

    fn get_legal_moves(&self) -> Vec<Self::Move> {
        let mut moves = Vec::new();

        if self.game_state() != GameState::Ongoing {
            return moves;
        }

        for col in 0..Connect4::WIDTH {
            if (self.mask & Connect4::top_mask(col)) == 0 {
                moves.push((col as u16).into());
            }
        }

        moves
    }
}

#[allow(dead_code)]
impl Connect4 {
    pub const WIDTH: usize = 7;
    pub const HEIGHT: usize = 6;

    pub fn print(&self) {
        let mut grid = [["."; Connect4::WIDTH]; Connect4::HEIGHT];

        for col in 0..Connect4::WIDTH {
            #[allow(clippy::needless_range_loop)]
            for row in 0..Connect4::HEIGHT {
                let i = row + col * (Connect4::HEIGHT + 1);

                if ((self.current >> i) & 1) != 0 {
                    grid[row][col] = "X";
                }

                if (((self.current ^ self.mask) >> i) & 1) != 0 {
                    grid[row][col] = "O";
                }
            }
        }

        for row in (0..Connect4::HEIGHT).rev() {
            println!("{}", grid[row].join(""));
        }
        println!()
    }

    fn alignment(pos: u64) -> bool {
        // horizontal
        let mut m = pos & (pos >> (Connect4::HEIGHT + 1));
        if (m & (m >> (2 * (Connect4::HEIGHT + 1)))) > 0 {
            return true;
        }

        // diagonal 1
        m = pos & (pos >> Connect4::HEIGHT);
        if (m & (m >> (2 * Connect4::HEIGHT))) > 0 {
            return true;
        }

        // diagonal 2
        m = pos & (pos >> (Connect4::HEIGHT + 2));
        if (m & (m >> (2 * (Connect4::HEIGHT + 2)))) > 0 {
            return true;
        }

        // vertical;
        m = pos & (pos >> 1);
        if (m & (m >> 2)) > 0 {
            return true;
        }

        false
    }

    fn top_mask(col: usize) -> u64 {
        (1u64 << (Connect4::HEIGHT - 1)) << (col * (Connect4::HEIGHT + 1))
    }

    fn bottom_mask(col: usize) -> u64 {
        1u64 << (col * (Connect4::HEIGHT + 1))
    }
}
