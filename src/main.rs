mod game;
use game::Nim;

fn main() {
    let mut game = Nim::new(15, true);

    game.play();
}
