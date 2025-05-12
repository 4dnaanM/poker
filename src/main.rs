mod deck; 
mod game; 
mod player;

use game::Game;
fn main() {
    let mut game = Game::new(2,100);
    game.play_round(0);
    game.play_round(1);
    game.play_round(2);
}
