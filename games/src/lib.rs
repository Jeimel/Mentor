pub mod chess;
pub mod connect4;

use mentor::Game;

pub trait UCI {
    type Game: mentor::Game;
    const DEFAULT_POS: String;

    fn run(&mut self) {
        let mut pos = Self::Game::default();
        let params = mentor::helper::MctsParameter::default();
        let mut search = mentor::search::Search::new(pos, 50_000);

        loop {
            let mut input = String::new();
            let bytes_read = std::io::stdin().read_line(&mut input).unwrap();

            if bytes_read == 0 {
                break;
            }

            let commands = input.split_whitespace().collect::<Vec<_>>();
            let command = *commands.first().unwrap_or(&"oops");
            match command {
                "uci" => {
                    println!("id name mentor");
                    println!("id author Felix Jablinski");
                    self.options();
                }
                "ucinewgame" => pos = Self::Game::default(),
                "isready" => println!("readyok"),
                "quit" => std::process::exit(0),
                "position" => self.position(&mut pos, commands),
                "go" => self.go(&mut pos, &mut search, &params, commands),
                "display" => println!("{}", pos),
                _ => {}
            }
        }
    }

    fn position(&mut self, pos: &mut Self::Game, commands: Vec<&str>) {
        let mut moves_start = false;

        for command in commands {
            if moves_start {
                let mov = command.parse::<u16>().unwrap();
                pos.make_move(mov.into());

                continue;
            }

            match command {
                "position" => {}
                "startpos" => {
                    *pos = Self::Game::default();
                    moves_start = true;
                }
                _ => todo!(),
            }
        }
    }

    fn options(&mut self);

    fn go(
        &mut self,
        pos: &mut Self::Game,
        search: &mut mentor::search::Search<Self::Game>,
        params: &mentor::helper::MctsParameter,
        commands: Vec<&str>,
    );
}
