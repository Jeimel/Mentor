pub mod helper;
pub mod search;
mod tree;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum GameState {
    #[default]
    Ongoing,
    Win,
    Draw,
    Loss,
}

pub trait Game: Clone + Copy + Default + Eq {
    type Move: std::fmt::Display + From<u16> + Into<u16> + Copy;

    fn side_to_move(&self) -> usize;

    fn game_state(&self) -> GameState;

    fn hash(&self) -> u64;

    fn get_value(&mut self) -> f32;

    fn get_policies(&mut self, moves: &[Self::Move]) -> Vec<f32>;

    fn make_move(&mut self, mov: Self::Move);

    fn get_legal_moves(&self) -> Vec<Self::Move>;
}
