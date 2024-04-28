use super::edge::Edge;
use crate::{Game, GameState};

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
    const C: f32 = 1.41;

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

    pub fn actions(&self) -> &Vec<Edge> {
        &self.actions
    }

    pub fn action(&mut self, index: usize) -> &mut Edge {
        &mut self.actions[index]
    }

    pub fn visits(&self) -> f32 {
        self.visits
    }

    pub fn wins(&self) -> f32 {
        self.wins
    }

    pub fn uct(&self, n: f32) -> f32 {
        if self.visits == 0.0 {
            return 0.0;
        }

        (self.wins / self.visits) + Node::C * (n.ln() / self.visits).sqrt()
    }

    pub fn expand<G: Game>(&mut self, pos: &G) {
        assert!(!self.is_expanded());

        let moves = pos.get_moves();
        for i in 0..moves.len() {
            self.actions.push(Edge::new(moves[i].into()));
        }
    }

    pub fn is_expanded(&self) -> bool {
        !self.actions.is_empty()
    }

    pub fn is_terminal(&self) -> bool {
        self.state != GameState::Ongoing
    }

    pub fn next_edge(&mut self) -> Option<&mut Edge> {
        assert!(self.is_expanded());

        for edge in &mut self.actions {
            if edge.ptr() == -1 {
                return Some(edge);
            }
        }

        None
    }

    pub fn propagate(&mut self, reward: f32) {
        self.visits += 1.0;
        self.wins += reward;
    }
}
