use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Instant,
};

use crate::{
    helper::{MctsParameter, SearchSettings},
    tree::Tree,
    Game, GameState,
};

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

    pub fn run(
        &mut self,
        pos: Option<G>,
        setings: &SearchSettings,
        params: &MctsParameter,
        abort: &AtomicBool,
        uci: bool,
    ) -> G::Move {
        let timer = Instant::now();

        self.tree.subtree::<G>(&self.root, &pos);
        if let Some(pos) = pos {
            self.root = pos;
        }

        let mut nodes = 0;
        loop {
            let mut root = self.root;

            self.execute_iteration(self.tree.root(), &mut root, params);
            if self.tree[self.tree.root()].is_terminal() {
                break;
            }

            nodes += 1;
            if nodes >= setings.max_nodes {
                break;
            }

            if abort.load(Ordering::Relaxed) {
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
                if edge.ptr() == -1 {
                    return (-1f32, edge.mov());
                }

                if uci {
                    println!(
                        "move {} n {} w {} q {}",
                        edge.mov(),
                        self.tree[edge.ptr()].visits(),
                        self.tree[edge.ptr()].value(),
                        self.tree[edge.ptr()].q()
                    );
                }

                (self.tree[edge.ptr()].visits(), edge.mov())
            })
            .max_by(|(a, _), (b, _)| a.total_cmp(b))
            .map(|(_, mov)| mov)
            .unwrap()
            .into()
    }

    pub fn execute_iteration(&mut self, index: i32, pos: &mut G, params: &MctsParameter) -> f32 {
        let reward = if self.tree[index].visits() == 0.0 || self.tree[index].is_terminal() {
            self.get_utility(index, pos)
        } else {
            if self.tree[index].is_not_expanded() {
                self.tree[index].expand(pos);
            }

            let action = self.pick_action(index, params);
            let edge = self.tree.edge(index, action);

            let mut edge_ptr = edge.ptr();

            pos.make_move(edge.mov().into());

            if edge.ptr() == -1 {
                edge_ptr = self.tree.add(pos.game_state(), pos.hash(), index);
                self.tree.edge_mut(index, action).set_ptr(edge_ptr);
            }

            self.execute_iteration(edge_ptr, pos, params)
        };

        let reward = -reward;
        self.tree.propagate(index, reward);

        reward
    }

    fn pick_action(&mut self, index: i32, params: &MctsParameter) -> usize {
        let node = &self.tree[index];

        let expl = params.cpuct(node) * node.visits().sqrt();

        let mut best = 0;
        let mut max = f32::NEG_INFINITY;

        let mut proven_win = true;

        for (i, action) in node.actions().iter().enumerate() {
            if action.ptr() == -1 {
                return i;
            }

            let state = self.tree[action.ptr()].game_state();
            if state == GameState::Loss {
                self.tree[index].set_game_state(GameState::Win);
                return i;
            }

            if state != GameState::Win {
                proven_win = false;
            }

            let u = expl * action.policy() / (1.0 + self.tree[action.ptr()].visits());
            let uct = self.tree[action.ptr()].q() + u;

            if max < uct {
                best = i;
                max = uct;
            }
        }

        if proven_win {
            self.tree[index].set_game_state(GameState::Loss);
        }

        best
    }

    fn get_utility(&self, index: i32, pos: &mut G) -> f32 {
        match self.tree[index].game_state() {
            GameState::Ongoing => pos.get_value(),
            GameState::Win => 1.0,
            GameState::Draw => 0.0,
            GameState::Loss => -1.0,
        }
    }
}
