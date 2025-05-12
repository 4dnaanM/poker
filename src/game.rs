use crate::deck::{Deck, Card};
use crate::player::{Player,Action,PlayerState};
pub struct Game {
    players: Vec<Player>, 
    small_blind: u32,
    big_blind: u32,
    buyin: u32
}

impl Game {

    pub fn new(n_players: u32, buyin: u32) -> Game {
        
        assert!( n_players < 23 );

        let mut players = Vec::new();
        for i in 0..n_players { 
            let player = Player::new(format!("Player {}", i + 1), buyin);
            players.push(player);
        }

        Game { players, small_blind: 1, big_blind: 2, buyin}
    }

    fn showdown(&mut self, community_cards: [Card;5]) -> usize {
        println!("Showdown");
        // for player in self.players.iter_mut() {
        //     player.display();
        // }
        self.players.retain(|player| player.chips > 0);
        for player in self.players.iter_mut() {
            player.reset();
        }
        return 0;
    }

    pub fn play_round(&mut self, dealer: usize){
        
        let revealed_card_numbers = [0,3,4,5];
        // let street_str = ["","Flop","Turn","River"];
        
        let mut deck = Deck::new();
        let mut action: Vec<Vec<Action>> = Vec::new(); 
        
        let n_players = self.players.len();
        if n_players==1  {return;} 
        for i in 0..2*n_players {
            let idx = (dealer + 1 + i) % n_players;
            self.players[idx].deal_card(deck.deal().unwrap());
        }


        for player in &self.players{
            println!("{}'s hand:",player.name);
            Deck::print_cards(&player.hand);
        }

        self.players[ (dealer+1) % n_players ].bet_blind(self.small_blind);
        println!("{} bet {}",self.players[ (dealer+1) % n_players ].name, self.small_blind);
        
        self.players[ (dealer+2) % n_players ].bet_blind(self.big_blind);
        println!("{} bet {}",self.players[ (dealer+2) % n_players ].name, self.big_blind);

        action.push(vec![Action::Bet(self.small_blind),Action::Bet(self.big_blind)]);

        let community_cards = std::array::from_fn(|_| deck.deal().unwrap());

        let mut n_folded = 0; 
        let mut n_all_in = 0; 
        let mut street = 0; 

        let mut pot = 0; 
        
        'street: loop {
            let revealed_upto = revealed_card_numbers[street];

            if revealed_upto!=0 {
                Deck::print_cards(&community_cards[0..revealed_upto]);
            }

            let (mut agreed_players, mut bet, mut idx) = if street != 0 {
                action.push(Vec::new());
                (0,0,1)
            } else {(1,self.big_blind,3)};
            
            while agreed_players + n_all_in + n_folded < n_players {
                
                // println!("Street: {},Agreed: {}, All In: {}, Folded: {}",street,agreed_players,n_all_in,n_folded);
    
                let player = &mut self.players[(idx+dealer)%n_players];
                // player.display(); 
                if player.state != PlayerState::Active {
                    idx = (idx + 1) % n_players;
                    continue; 
                }
                
                let player_action = player.act(pot, &community_cards[..revealed_upto], bet, &action); 
                
                match player_action {
                    Action::Check => {
                        agreed_players+=1; 
                        println!("{} checked",player.name);
                        action[street].push(Action::Check);
                    },
                    Action::Fold => {
                        n_folded += 1; 
                        println!("{} folded",player.name);
                        action[street].push(Action::Fold);
                    },
                    Action::Call => {
                        agreed_players+=1; 
                        pot += bet; 
                        println!("{} called",player.name);
                        action[street].push(Action::Call);
                    },
                    Action::Bet(player_bet) => {
                        agreed_players = 1;
                        bet = player_bet;
                        pot += bet; 
                        println!("{} bet {}",player.name, player_bet);
                        action[street].push(Action::Bet(player_bet));
                    },
                    Action::AllIn(player_bet) => {
                        n_all_in += 1;
                        if player_bet > bet {
                            agreed_players = 1;
                        }
                        bet = player_bet; 
                        pot += bet; 
                        println!("{} went all in for {}",player.name, player_bet);
                        action[street].push(Action::AllIn(player_bet));
                    }
                }
                idx = (idx+1) % n_players; 
            }
            street +=1 ;
            if street > 3{ break 'street } 
        }

        self.showdown(community_cards);

    }

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_initialization() {
        let game = Game::new(4,500);
        assert_eq!(game.players.len(), 4);
        assert_eq!(game.small_blind, 1);
        assert_eq!(game.big_blind, 2);
    }

    #[test]
    #[should_panic]
    fn test_game_initialization_too_many_players() {
        Game::new(23,500); // Should panic because the maximum number of players is 22
    }

    #[test]
    fn test_play_round() {
        let mut game = Game::new(4,500);
        game.play_round(0);
        // Add assertions here based on expected game state after a round
    }

    #[test]
    fn test_showdown() {
        let mut game = Game::new(4,500);
        let mut deck = Deck::new(); 
        let community_cards = [
            deck.deal().unwrap(),
            deck.deal().unwrap(),
            deck.deal().unwrap(),
            deck.deal().unwrap(),
            deck.deal().unwrap(),
        ];
        game.showdown(community_cards);
    }

    #[test]
    fn test_player_bets() {
        let mut game = Game::new(3,500);
        let dealer = 0;

        game.players[1].bet_blind(game.small_blind);
        assert_eq!(game.players[1].chips, 499);

        game.players[2].bet_blind(game.big_blind);
        assert_eq!(game.players[2].chips, 498);

        game.play_round(dealer);
    }
    #[test]
    fn test_play_multiple_rounds(){
        let mut game = Game::new(3,500);
        for round in 0..=100{
            game.play_round(round);
        }
    }
}
