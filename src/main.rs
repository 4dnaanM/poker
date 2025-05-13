mod deck; 
mod game; 
mod player;

use deck::Deck;
use game::Game;
fn main() {
    let mut game = Game::new(10,100);
    // Deck::print_cards(Deck::new().deck);
    game.play_round(0);
    // game.play_round(1);
    // game.play_round(2);
}
