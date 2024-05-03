pub mod tictactoe;

use mentor::{
    search::{Search, SearchSettings},
    Game, GameState,
};

use crate::tictactoe::TicTacToe;

fn main() {
    let mut game = TicTacToe {
        side_to_move: 0,
        grid: [0b_000_000_000, 0b_000_000_000],
    };

    let mut search = Search::new(game);
    while game.game_state() == GameState::Ongoing {
        let settings = SearchSettings {
            max_time: Some(1000),
            max_nodes: 80_000,
        };

        let mov: tictactoe::TicTacToeMove = match game.side_to_move {
            0 | 1 => search.run(Some(game), settings),
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
        game.print();
    }

    println!("{:?}", game.game_state());
}
