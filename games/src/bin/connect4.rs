use games::connect4::Connect4;
use mentor::{
    helper::{MctsParameter, SearchSettings},
    search::Search,
    Game, GameState,
};

fn main() {
    let mut game = Connect4::default();

    let params = MctsParameter::new(1.41, 0.25);
    let settings = SearchSettings {
        max_time: Some(2500),
        max_nodes: usize::MAX,
    };

    let mut search = Search::new(game, 50_000, params);
    while game.game_state() == GameState::Ongoing {
        let mov = if game.side_to_move() == 0 {
            search.run(Some(game), &settings)
        } else {
            let mut input_line = String::new();
            std::io::stdin()
                .read_line(&mut input_line)
                .expect("Failed to read line");

            input_line
                .trim()
                .parse::<u16>()
                .expect("Input not an integer")
                .into()
        };

        game.make_move(mov);
        println!("{game}");
    }

    println!("{:?}", game.game_state());
}
