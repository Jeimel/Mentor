use super::edge::Edge;
use crate::{Game, GameState};

#[derive(Clone)]
pub struct Node {
    parent: i32,
    state: GameState,
    hash: u64,
    actions: Vec<Edge>,
    wins: f32,
    visits: f32,
}

impl Node {
    pub fn new(state: GameState, hash: u64, parent: i32) -> Self {
        Node {
            parent,
            state,
            hash,
            actions: Vec::new(),
            wins: 0.0,
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

    pub fn wins(&self) -> f32 {
        self.wins
    }

    pub fn expand<G: Game>(&mut self, pos: &G) {
        assert!(self.is_not_expanded());

        for mov in pos.get_legal_moves() {
            self.actions.push(Edge::new(mov.into()));
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
        self.wins += reward;
    }
}
