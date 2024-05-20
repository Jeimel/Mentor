use games::connect4::Connect4;
use mentor::{
    search::{Search, SearchSettings},
    Game, GameState,
};

fn main() {
    let mut game = Connect4 {
        side_to_move: false,
        current: 0,
        mask: 0,
    };

    let mut search = Search::new(game, 50_000);
    while game.game_state() == GameState::Ongoing {
        let settings = SearchSettings {
            max_time: Some(2500),
            max_nodes: usize::MAX,
        };

        let mov = match game.side_to_move {
            false => search.run(Some(game), settings),
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
