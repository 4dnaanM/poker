mod deck;
mod game;
mod player;
mod utils;

use game::Game;
fn main() {
    let mut game = Game::new(5, 10000);
    // Deck::print_cards(Deck::new().deck);
    for round in 0..10000 {
        game.play_round(round);
    }
}
