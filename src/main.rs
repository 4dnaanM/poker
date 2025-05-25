mod deck; 
mod game; 
mod player;

use deck::Deck;
use game::Game;
fn main() {
    let mut game = Game::new(2,100);
    // Deck::print_cards(Deck::new().deck);
    for round in 0..3 {
        game.play_round(round);
    }
}
