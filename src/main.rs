mod deck; 
mod game; 
mod player;

use deck::Deck;
use game::Game;
fn main() {
    let mut game = Game::new(2,100);
    Deck::print_cards(Deck::new().deck);
    for round in 0..100 {
        game.play_round(round);
    }
    
    // game.play_round(1);
    // game.play_round(2);
}
