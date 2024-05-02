pub mod tictactoe;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum GameState {
    #[default]
    Ongoing,
    Win,
    Draw,
    Loss,
}

pub trait Game: Clone + Copy + Default {
    type Move: std::fmt::Display + From<u16> + Into<u16> + Copy;

    fn side_to_move(&self) -> usize;

    fn game_state(&self) -> GameState;

    fn make_move(&mut self, mov: Self::Move);

    fn get_moves(&self) -> Vec<Self::Move>;
}
