pub mod edge;
pub mod node;
pub mod tree;

use crate::{mcts::tree::Tree, Game, GameState};

pub struct Search<G: Game> {
    root: G,
    tree: Tree,
}

impl<G: Game> Search<G> {
    pub fn new(position: G) -> Self {
        Search {
            root: position,
            tree: Tree::new(position.game_state()),
        }
    }

    pub fn run(&mut self) -> G::Move {
        self.tree[0].expand(&self.root.clone());

        for _ in 0..1_600 {
            self.execute_one_iteration();
        }

        self.tree[0]
            .actions()
            .iter()
            .map(|edge| (self.tree[edge.ptr()].visits(), edge.mov()))
            .max_by(|(a, _), (b, _)| a.total_cmp(b))
            .map(|(_, mov)| mov)
            .unwrap()
            .into()
    }

    pub fn execute_one_iteration(&mut self) {
        let mut pos = self.root.clone();

        // 1. Select leaf node
        let index = self.tree.select(&mut pos);

        // 1.1 Backpropagate early if terminal state is reached in leaf
        if pos.game_state() != GameState::Ongoing {
            self.backpropagate(pos.game_state(), index);
            return;
        }

        // 2. Expand leaf node
        self.tree[index].expand(&mut pos);

        // 3. Simulate game with random plays
        Search::simulate(&mut pos);

        // 4. Backpropagate result
        self.backpropagate(pos.game_state(), index);
    }

    pub fn backpropagate(&mut self, state: GameState, index: i32) {
        let mut index = index;

        let mut reward = match state {
            GameState::Win => 1.0,
            GameState::Draw => 0.5,
            GameState::Loss => 0.0,
            GameState::Ongoing => panic!(),
        };

        while index != -1 {
            self.tree[index].propagate(reward);

            reward = 1.0 - reward;
            index = self.tree[index].parent();
        }
    }

    pub fn simulate(pos: &mut G) {
        while pos.game_state() == GameState::Ongoing {
            let moves = pos.get_moves();
            let index = (rand::random::<f32>() * moves.len() as f32).floor() as usize;

            pos.make_move(moves[index]);
        }
    }
}
