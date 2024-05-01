use crate::games::{Game, GameState};

use super::edge::Edge;

#[derive(Clone)]
pub struct Node {
    parent: i32,
    state: GameState,
    actions: Vec<Edge>,
    wins: f32,
    visits: f32,
}

impl From<GameState> for Node {
    fn from(state: GameState) -> Self {
        Node {
            parent: -1,
            state,
            actions: Vec::new(),
            wins: 0.0,
            visits: 0.0,
        }
    }
}

impl Node {
    const C: f32 = 2.0;

    pub fn new(state: GameState, parent: i32) -> Self {
        Node {
            parent,
            state,
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

    pub fn mut_edge(&mut self, index: usize) -> &mut Edge {
        &mut self.actions[index]
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

    pub fn uct(&self, n: f32) -> f32 {
        (-self.wins / self.visits) + (Node::C * n.ln() / self.visits).sqrt()
    }

    pub fn expand<G: Game>(&mut self, pos: &G) {
        assert!(self.is_not_expanded());

        for mov in pos.get_moves() {
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
