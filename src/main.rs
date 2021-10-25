mod board;

fn main() {
    let game = board::Game::new();
    game.draw();
    println!("Hello, world!");
}
