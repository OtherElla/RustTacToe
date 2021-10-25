mod board;

use board::{Game};

fn main() {
    let game = Game::new();
    game.draw();
}
