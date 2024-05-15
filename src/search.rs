use std::time::Instant;

use crate::{
    tree::{node::Node, Tree},
    Game,
};

pub struct SearchSettings {
    pub max_time: Option<u128>,
    pub max_nodes: usize,
}

pub struct Search<G: Game> {
    root: G,
    tree: Tree,
}

impl<G: Game> Search<G> {
    pub fn new(pos: G, capacity: usize) -> Self {
        Search {
            root: pos,
            tree: Tree::new(capacity),
        }
    }

    pub fn run(&mut self, pos: Option<G>, setings: SearchSettings) -> G::Move {
        let timer = Instant::now();

        self.tree.subtree::<G>(&self.root, &pos);
        if let Some(pos) = pos {
            self.root = pos;
        }

        let mut nodes = 0;
        loop {
            let mut root = self.root;

            self.execute_iteration(self.tree.root(), &mut root);

            nodes += 1;
            if nodes >= setings.max_nodes {
                break;
            }

            match setings.max_time {
                Some(time) if timer.elapsed().as_millis() >= time => break,
                _ => continue,
            };
        }

        self.tree[self.tree.root()]
            .actions()
            .iter()
            .map(|edge| {
                println!(
                    "Move {} Score {} Visits {}",
                    edge.mov(),
                    self.tree[edge.ptr()].wins(),
                    self.tree[edge.ptr()].visits()
                );

                (self.tree[edge.ptr()].visits(), edge.mov())
            })
            .max_by(|(a, _), (b, _)| a.total_cmp(b))
            .map(|(_, mov)| mov)
            .unwrap()
            .into()
    }

    pub fn execute_iteration(&mut self, index: i32, pos: &mut G) -> f32 {
        let reward = if self.tree[index].visits() == 0.0 || self.tree[index].is_terminal() {
            self.get_utility(index, pos)
        } else {
            if self.tree[index].is_not_expanded() {
                self.tree[index].expand(pos);
            }

            let action = self.pick_action(index);
            let edge = self.tree.edge(index, action);

            let mut edge_ptr = edge.ptr();

            pos.make_move(edge.mov().into());

            if edge.ptr() == -1 {
                edge_ptr = self
                    .tree
                    .add(Node::new(pos.game_state(), pos.hash(), index));
                self.tree.edge_mut(index, action).set_ptr(edge_ptr);
            }

            self.execute_iteration(edge_ptr, pos)
        };

        let reward = 1.0 - reward;
        self.tree.propagate(index, reward);

        reward
    }

    fn pick_action(&self, index: i32) -> usize {
        let node = &self.tree[index];

        let expl = 1.41 * (node.visits().ln()).sqrt();

        let mut best = 0;
        let mut max = f32::NEG_INFINITY;

        for (i, action) in node.actions().iter().enumerate() {
            if action.ptr() == -1 {
                return i;
            }

            let uct = self.tree[action.ptr()].wins() / self.tree[action.ptr()].visits()
                + expl / self.tree[action.ptr()].visits().sqrt();

            if max < uct {
                best = i;
                max = uct;
            }
        }

        best
    }

    fn get_utility(&self, index: i32, pos: &mut G) -> f32 {
        match self.tree[index].game_state() {
            crate::GameState::Ongoing => pos.get_value(),
            crate::GameState::Win => 1.0,
            crate::GameState::Draw => 0.5,
            crate::GameState::Loss => 0.0,
        }
    }
}
