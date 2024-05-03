pub mod tictactoe;

use mentor::{search::Search, Game, GameState};

use crate::tictactoe::TicTacToe;

fn main() {
    let mut game = TicTacToe {
        side_to_move: 0,
        grid: [0b_000_000_000, 0b_000_000_000],
    };

    while game.game_state() == GameState::Ongoing {
        let mut search = Search::new(game);

        let mov = match game.side_to_move {
            0 | 1 => search.run(),
            _ => {
                let mut input_line = String::new();
                std::io::stdin()
                    .read_line(&mut input_line)
                    .expect("Failed to read line");

                input_line
                    .trim()
                    .parse::<u16>()
                    .expect("Input not an integer")
                    .into()
            }
        };

        game.make_move(mov);
    }

    println!("{:?}", game.game_state());
}
