mod deck; 
mod game; 
mod player;

use game::Game;
fn main() {
    let mut game = Game::new(2);
    game.play_round(0);
}
