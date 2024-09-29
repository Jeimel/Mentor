use std::sync::atomic::{AtomicBool, Ordering};

use mentor::{
    mcts::{params::SearchParameter, settings::SearchSettings, Search},
    Game, GameState,
};

use crate::{rand::Rand, AtomicStats};

pub struct DatagenThread<G: Game> {
    params: SearchParameter,
    settings: SearchSettings,
    positions: Vec<(G, f32)>,
    games: usize,
}

impl<G: Game> DatagenThread<G> {
    pub fn new(params: SearchParameter, settings: SearchSettings) -> Self {
        DatagenThread {
            params,
            settings,
            positions: Vec::new(),
            games: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.positions.len()
    }

    pub fn games(&self) -> usize {
        self.games
    }

    pub fn run(&mut self, abort: &AtomicBool, stats: &AtomicStats) {
        while !abort.load(Ordering::Relaxed) {
            let positions = self.run_game();

            self.games += 1;
            stats.update(positions);
        }

        for (pos, score) in &self.positions {
            println!("{}\n{}", pos, score);
        }
    }

    fn run_game(&mut self) -> usize {
        let mut pos = G::default();

        let mut rand = Rand::default();
        for _ in 0..(rand.random_range(0, 8)) {
            let moves = pos.get_legal_moves();

            pos.make_move(moves[rand.random_range(0, moves.len())]);
        }

        let abort = AtomicBool::new(false);

        let mut positions = Vec::with_capacity(42);

        while let GameState::Ongoing = pos.game_state() {
            positions.push(pos);
            let mut search = Search::new(pos, 50_000);

            let mov = search.run(Some(pos), &self.settings, &self.params, &abort, false);
            pos.make_move(mov);
        }

        let mut result = match pos.game_state() {
            GameState::Draw => 0.0,
            GameState::Loss => 1.0,
            GameState::Win | GameState::Ongoing => panic!(),
        };

        for i in (0..positions.len()).rev() {
            self.positions.push((positions[i], result));

            result = -result;
        }

        positions.len()
    }
}
