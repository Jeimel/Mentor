#[allow(unused_imports)]
use games::GameProtocol;

fn main() {
    #[cfg(feature = "connect4")]
    games::connect4::Connect4Protocol {}.run();
}
