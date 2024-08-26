mod board;
mod moves;

use std::fmt::{self};

use board::Board;
use mentor::{
    helper::{MctsParameter, SearchSettings},
    search::Search,
    Game, GameState,
};
use moves::Move;

use crate::UCI;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Connect4 {
    board: Board,
}

impl fmt::Display for Connect4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.board.print())
    }
}

impl Game for Connect4 {
    type Move = Move;

    fn side_to_move(&self) -> usize {
        usize::from(self.board.side_to_move())
    }

    fn game_state(&self) -> GameState {
        if Board::alignment(self.board.current() ^ self.board.mask()) {
            return GameState::Loss;
        }

        if Board::alignment(self.board.current()) {
            return GameState::Win;
        }

        if self.board.mask().count_ones() == (Board::WIDTH * Board::HEIGHT) as u32 {
            return GameState::Draw;
        }

        GameState::Ongoing
    }

    fn hash(&self) -> u64 {
        self.board.current() + self.board.mask()
    }

    fn get_value(&mut self) -> f32 {
        let mut pos = *self;

        let side_to_move = pos.side_to_move();

        let mut iterations = 0;
        while pos.game_state() == GameState::Ongoing {
            let moves = pos.get_legal_moves();
            let index = (rand::random::<f32>() * moves.len() as f32).floor() as usize;

            pos.make_move(moves[index]);
            iterations += 1;
        }

        let reward = 1.0 - (iterations as f32 / 100.0);
        match pos.game_state() {
            GameState::Draw => 0.0,
            _ if side_to_move == pos.side_to_move() => -reward,
            _ => reward,
        }
    }

    fn get_policies(&mut self, moves: &[Self::Move]) -> Vec<f32> {
        let mut policies = Vec::with_capacity(moves.len());

        for mov in moves {
            let policy: f32 = match mov.0 {
                0 | 6 => 1.0,
                1 | 5 => 2.0,
                2 | 4 => 4.0,
                3 => 7.0,
                _ => panic!(),
            };

            policies.push(policy.exp());
        }

        let sum: f32 = policies.iter().sum();
        policies.iter().map(|&x| x / sum).collect()
    }

    fn make_move(&mut self, mov: Self::Move) {
        self.board.make_move(mov.into());
    }

    fn get_legal_moves(&self) -> Vec<Self::Move> {
        if self.game_state() != GameState::Ongoing {
            return Vec::new();
        }

        self.board.get_moves().into_iter().map(Into::into).collect()
    }
}

pub struct Connect4Interface;

impl UCI for Connect4Interface {
    type Game = Connect4;
    const DEFAULT_POS: String = String::new();

    fn options(&mut self) {}

    fn go(
        &mut self,
        pos: &mut Self::Game,
        search: &mut Search<Self::Game>,
        params: &MctsParameter,
        _: Vec<&str>,
    ) {
        let settings = SearchSettings {
            max_time: Some(2500),
            max_nodes: usize::MAX,
        };

        let mov = search.run(Some(*pos), &settings, params);
        println!("bestmove {}", mov);
    }
}
