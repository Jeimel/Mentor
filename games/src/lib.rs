pub mod chess;
pub mod connect4;

use mentor::Game;

fn handle_input<F: FnMut(&str, Vec<&str>)>(mut f: F) {
    loop {
        let mut input = String::new();
        let bytes_read = std::io::stdin().read_line(&mut input).unwrap();

        if bytes_read == 0 {
            break;
        }

        let commands = input.split_whitespace().collect::<Vec<_>>();
        let command = *commands.first().unwrap_or(&"oops");

        f(command, commands);
    }
}

pub trait GameProtocol {
    type Game: mentor::Game;

    const NAME: &'static str;
    const NEW_GAME: &'static str;
    const NOTATION: &'static str;

    const DEFAULT_POS: String;

    fn run(&mut self) {
        let mut pos = Self::Game::default();
        let params = mentor::helper::MctsParameter::default();
        let mut search = mentor::search::Search::new(pos, 50_000);

        handle_input(|command, commands| match command {
            "quit" => std::process::exit(0),
            "setoption" => todo!(),
            "position" => self.position(&mut pos, commands),
            "isready" => println!("readyok"),
            "go" => {
                self.go(&mut pos, &mut search, &params, commands);
            }
            "d" => println!("{}", pos),
            _ if command == Self::NAME => {
                println!("id name mentor");
                println!("id author Felix Jablinski");
                self.options();
            }
            _ if command == Self::NEW_GAME => pos = Self::Game::default(),
            _ => {}
        })
    }

    fn search_input(&mut self, abort: &std::sync::atomic::AtomicBool) {
        handle_input(|command, _| match command {
            "quit" => std::process::exit(0),
            "isready" => println!("readyok"),
            "stop" => abort.store(true, std::sync::atomic::Ordering::Relaxed),
            _ if command == Self::NAME => {
                println!("id name mentor");
                println!("id author Felix Jablinski");
                self.options();
            }
            _ => {}
        });
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
                _ if command == Self::NOTATION => {}
                _ => {
                    *pos = Self::Game::from_str(command);
                    moves_start = true;
                }
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
