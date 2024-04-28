use mentor::{mcts::Search, TicTacToe};

fn main() {
    let game = TicTacToe {
        turn: 0,
        grid: [0b_000_000_000, 0b_000_000_000],
    };

    let mut search = Search::new(game);
    println!("{}", search.run());
}
