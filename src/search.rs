use std::time::Instant;

use crate::{
    helper::{MctsParameter, SearchSettings},
    tree::{node::Node, Tree},
    Game,
};

pub struct Search<G: Game> {
    root: G,
    tree: Tree,
    params: MctsParameter,
}

impl<G: Game> Search<G> {
    pub fn new(pos: G, capacity: usize, params: MctsParameter) -> Self {
        Search {
            root: pos,
            tree: Tree::new(capacity),
            params,
        }
    }

    pub fn run(&mut self, pos: Option<G>, setings: &SearchSettings) -> G::Move {
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
                    self.tree[edge.ptr()].value(),
                    self.tree[edge.ptr()].visits()
                );

                (self.tree[edge.ptr()].value(), edge.mov())
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

        let reward = -reward;
        self.tree.propagate(index, reward);

        reward
    }

    fn pick_action(&self, index: i32) -> usize {
        let node = &self.tree[index];

        let expl = self.params.cpuct(node) * node.visits().sqrt();

        let mut best = 0;
        let mut max = f32::NEG_INFINITY;

        for (i, action) in node.actions().iter().enumerate() {
            if action.ptr() == -1 {
                return i;
            }

            let u = expl * action.policy() / (1.0 + self.tree[action.ptr()].visits());
            let uct = self.tree[action.ptr()].q() + u;

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
            crate::GameState::Draw => 0.0,
            crate::GameState::Loss => -1.0,
        }
    }
}
