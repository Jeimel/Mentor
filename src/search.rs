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
    pub fn new(pos: G) -> Self {
        Search {
            root: pos,
            tree: Tree::new(),
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

        println!("{}", self.tree.len());

        self.tree[self.tree.root()]
            .actions()
            .iter()
            .map(|edge| {
                println!(
                    "Move {} Score {} Wins {} Visits {}",
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
            let reward = pos.get_value();
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
            let node = Node::new(pos.game_state(), index);
            new_index = self.tree.add(node);
            self.tree[index].mut_edge(edge).set_ptr(new_index);
        }

        let reward = -self.execute_iteration(new_index, pos);
        self.tree[index].propagate(reward);

        reward
    }
}
