mod board;
mod moves;
mod types;
mod util;

use self::{board::Board, moves::Move};
use mentor::Game;

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct Chess {
    board: Board,
}

impl Game for Chess {
    type Move = Move;

    fn side_to_move(&self) -> usize {
        usize::from(self.board.side_to_move)
    }

    fn game_state(&self) -> mentor::GameState {
        todo!()
    }

    fn hash(&self) -> u64 {
        todo!()
    }

    fn get_value(&mut self) -> f32 {
        todo!()
    }

    fn make_move(&mut self, _: Self::Move) {
        todo!()
    }

    fn get_legal_moves(&self) -> Vec<Self::Move> {
        self.board.gen_moves()
    }
}
