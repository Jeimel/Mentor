#[allow(unused_imports)]
use games::UCI;

fn main() {
    #[cfg(feature = "connect4")]
    games::connect4::Connect4Interface {}.run()
}
