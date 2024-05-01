pub mod edge;
pub mod node;
pub mod tree;

use crate::{mcts::tree::Tree, Game, GameState};

use self::node::Node;

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
        for _ in 0..5_000 {
            self.execute_iteration(0, &mut self.root.clone());
        }

        self.tree[0]
            .actions()
            .iter()
            .map(|edge| {
                println!(
                    "{} Score {} Wins {} Visits {}",
                    edge.mov(),
                    -self.tree[edge.ptr()].wins() / self.tree[edge.ptr()].visits(),
                    self.tree[edge.ptr()].wins(),
                    self.tree[edge.ptr()].visits()
                );

                (
                    -self.tree[edge.ptr()].wins() / self.tree[edge.ptr()].visits(),
                    edge.mov(),
                )
            })
            .max_by(|(a, _), (b, _)| a.total_cmp(b))
            .map(|(_, mov)| mov)
            .unwrap()
            .into()
    }

    pub fn execute_iteration(&mut self, index: i32, pos: &mut G) -> f32 {
        if self.tree[index].visits() == 0.0 {
            let reward = Search::simulate(pos);
            self.tree[index].propagate(reward);

            return reward;
        }

        if self.tree[index].is_not_expanded() {
            self.tree[index].expand(pos);
        }

        if self.tree[index].is_terminal() {
            return self.tree[index].wins() / self.tree[index].visits();
        }

        let n = self.tree[index].visits();
        let (edge, mut new_index, mov) = self.tree[index]
            .actions()
            .iter()
            .enumerate()
            .map(|(index, edge)| {
                if edge.ptr() == -1 {
                    return (index, f32::INFINITY, -1, edge.mov());
                }

                (index, self.tree[edge.ptr()].uct(n), edge.ptr(), edge.mov())
            })
            .max_by(|(_, a, _, _), (_, b, _, _)| a.total_cmp(b))
            .map(|(edge, _, index, mov)| (edge, index, mov))
            .unwrap();

        pos.make_move(mov.into());

        if new_index == -1 {
            new_index = self.tree.len();

            let node = Node::new(pos.game_state(), index);
            self.tree.add(node);
            self.tree[index].mut_edge(edge).set_ptr(new_index);
        }

        let reward = -self.execute_iteration(new_index, pos);
        self.tree[index].propagate(reward);

        reward
    }

    pub fn simulate(pos: &mut G) -> f32 {
        let mut reward = 0.0;

        while pos.game_state() == GameState::Ongoing {
            let moves = pos.get_moves();
            let index = (rand::random::<f32>() * moves.len() as f32).floor() as usize;

            pos.make_move(moves[index]);

            reward = 1.0 - reward;
        }

        if pos.game_state() == GameState::Draw {
            return 0.0;
        }

        if reward == 0.0 {
            return -1.0;
        }

        reward
    }
}
