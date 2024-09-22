fn main() {
    #[cfg(feature = "connect4")]
    datagen::run::<games::connect4::Connect4>();
}
