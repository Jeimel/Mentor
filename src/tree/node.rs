use super::edge::Edge;
use crate::{Game, GameState};

#[derive(Clone)]
pub struct Node {
    parent: i32,
    state: GameState,
    hash: u64,
    actions: Vec<Edge>,
    value: f32,
    visits: f32,
}

impl Node {
    pub fn new(state: GameState, hash: u64, parent: i32) -> Self {
        Node {
            parent,
            state,
            hash,
            actions: Vec::new(),
            value: 0.0,
            visits: 0.0,
        }
    }

    pub fn parent(&self) -> i32 {
        self.parent
    }

    pub fn game_state(&self) -> GameState {
        self.state
    }

    pub fn hash(&self) -> u64 {
        self.hash
    }

    pub fn actions_mut(&mut self) -> &mut Vec<Edge> {
        &mut self.actions
    }

    pub fn actions(&self) -> &Vec<Edge> {
        &self.actions
    }

    pub fn visits(&self) -> f32 {
        self.visits
    }

    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn q(&self) -> f32 {
        self.value / self.visits
    }

    pub fn expand<G: Game>(&mut self, pos: &mut G) {
        assert!(self.is_not_expanded());

        let moves = pos.get_legal_moves();
        let policies = pos.get_policies(&moves);

        if moves.len() != policies.len() {
            panic!("Number of moves doesn't match number of policies.")
        }

        for mov in moves {
            self.actions.push(Edge::new(mov.into()));
        }

        for (i, action) in self.actions.iter_mut().enumerate() {
            action.set_policy(policies[i]);
        }
    }

    pub fn is_not_expanded(&self) -> bool {
        self.actions.is_empty()
    }

    pub fn is_terminal(&self) -> bool {
        self.state != GameState::Ongoing
    }

    pub fn propagate(&mut self, reward: f32) {
        self.visits += 1.0;
        self.value += reward;
    }
}
