use std::vec;

use crate::deck::{Deck, Card};
use crate::player::{Action, Player, PlayerState};
use crate::utils::{Hand,HandComparator};

pub struct Game {
    players: Vec<Player>, 
    small_blind: u32,
    big_blind: u32,
    buyin: u32
}
enum BettingRoundName{
    Preflop, 
    Flop, 
    Turn, 
    River
}
struct BettingRoundParams{
    round: BettingRoundName,
    pot: i32,
    action: Vec<Action>,
    n_active: usize, 


}

impl Game {

    pub fn new(n_players: u32, buyin: u32) -> Game {
        
        assert!( n_players < 23 );

        let mut players = Vec::new();
        for i in 0..n_players { 
            let player = Player::new(i as usize,format!("Player {}", i + 1), buyin);
            players.push(player);
        }

        Game { players, small_blind: 1, big_blind: 2, buyin}
    }
    
    fn find_winner(community_cards: [Card;5], players: &Vec<Player>) -> usize {

        // havent implemented split pots

        if players.len() - players.iter().filter(|p| (p.state == PlayerState::Folded) ).count() == 1 {
            println!("Only one remaining player");    
        }
        
        let mut winners: Vec::<(usize,([Card;5],Hand))> = Vec::new();
        for player in players {

            if player.state== PlayerState::Folded { 
                continue;
            }
            
            let mut hand: Vec<Card> = Vec::new();
            hand.extend_from_slice(&community_cards);
            hand.extend_from_slice(&player.hand);

            let best_hand = HandComparator::best_hand(hand);

            // now that we have best hand of this person, compare with earlier best hand and replace; 
            if !winners.is_empty() {
                let (_, winner_hand) = winners[0];
                if best_hand.1 > winner_hand.1 {
                    winners.clear(); 
                    winners.push((player.id,best_hand));
                }
                else if best_hand.1 == winner_hand.1 {
                    // they have same 
                    // look through sorted cards and find first diff
                    let old = winners[0].1.0;
                    let new = best_hand.0;
                    for idx in 0..5 {
                        if new[idx].0 > old[idx].0 {
                            winners.clear(); 
                            winners.push((player.id,best_hand));
                            break; 
                        }
                        else if old[idx].0 > new[idx].0 {
                            break; 
                        }
                        else if idx == 4 {
                            winners.push((player.id,best_hand));
                        }
                    }
                }
            } else {winners.push((player.id,best_hand));}
        }
        println!("Winning Hand: ");
        Deck::print_cards(&winners[0].1.0);
        
        return winners[0].0; 
    }
      
    fn showdown(&mut self, community_cards: [Card;5], mut pots: Vec<u32>, all_in_player_ids: Vec<Vec<usize>>) -> usize {
        // for now just find one winner, fix later to do side pots
        let max_pot = pots.iter().sum();

        println!("Showdown");
        let mut pot_distributed = 0; 
        let mut players = self.players.clone(); 
        
        while pot_distributed < max_pot {
            let winner_id = Game::find_winner(community_cards, &players);
            let mut pot = 0; 
            for idx in 0..=3 {
                pot+=pots[idx];
                pots[idx] = 0;
                if all_in_player_ids[idx].contains(&winner_id) {
                    break; 
                }
            }
            let (idx, winner) = self.players
                .iter_mut()
                .enumerate()
                .find(|(_, player)| player.id == winner_id)
                .unwrap();
            winner.deal_chips(pot);
            players.remove(idx);
            
            pot_distributed+=pot;
            // println!("Side Pot: Player {} got {} chips", winner_id,pot);
        }

        self.players.retain(|player| player.chips > 0);
        for player in self.players.iter_mut() {
            player.reset();
        }
        return 0;
    }

    pub fn play_round(&mut self, dealer: usize){

        // Important Test: Can't allow raise if everyone else all in
        
        // print!("Stacks: ");
        // for player in &self.players {
        //     print!("{}: {}, ", player.name, player.chips);
        // }
        
        let revealed_card_numbers = [0,3,4,5];
        
        let mut deck = Deck::new();
        let mut action: Vec<Vec<Action>> = Vec::new(); 
        
        let n_players = self.players.len();
        if n_players<=1  {return;} 

        for i in 0..2*n_players {
            let idx = (dealer + 1 + i) % n_players;
            self.players[idx].deal_card(deck.deal().unwrap());
        }

        for player in &self.players{
            player.display();
            // println!("{}'s hand:",player.name);
            // Deck::print_cards(&player.hand);
        }

        let mut pots :Vec<u32> = vec![0,0,0,0]; 
        let mut all_in_player_ids : Vec<Vec<usize>> = vec![vec![],vec![],vec![],vec![]];
        let mut pot = 0; 

        self.players[ (dealer+1) % n_players ].bet_blind(self.small_blind);
        pot += self.small_blind; 
        println!("{} bet blind {}, current_bet: {}, pot: {}",self.players[ (dealer+1) % n_players ].name, self.small_blind, self.small_blind, pot);
        
        self.players[ (dealer+2) % n_players ].bet_blind(self.big_blind);
        pot += self.big_blind; 
        println!("{} bet blind {}, current_bet: {}, pot: {}",self.players[ (dealer+2) % n_players ].name, self.big_blind, self.big_blind, pot);

        action.push(vec![Action::Raise(self.small_blind),Action::Raise(self.small_blind)]);

        let community_cards = std::array::from_fn(|_| deck.deal().unwrap());
        let mut street = 0; 
        let mut current_bet = self.big_blind;

        'street: loop {
            let mut n_active = self.players.iter().filter(|p| p.state == PlayerState::Active).count(); 
            if n_active<=1 {break}; 
            // deck.burn_card(); // what
            
            let revealed_upto = revealed_card_numbers[street];

            if revealed_upto!=0 {
                Deck::print_cards(&community_cards[0..revealed_upto]);
            }

            let  mut idx = if street != 0 {
                action.push(Vec::new());
                1
            } else {3};

            let mut callers = 0;  
            let mut n_all_in_this_street = 0; 
            
            while callers + n_all_in_this_street < n_active {
    
                let player = &mut self.players[(idx+dealer)%n_players];
                if player.state != PlayerState::Active {
                    idx = (idx + 1) % n_players;
                    continue; 
                }
                
                let player_bet = player.bet; 
                let player_action = player.act(pot, &community_cards[..revealed_upto], current_bet-player_bet, &action); 
                
                match player_action {
                    Action::Check => {
                        callers+=1; 
                        println!("{} checked, current_bet: {}, pot: {}",player.name,current_bet, pot);
                        action[street].push(Action::Check);
                    },
                    Action::Fold => {
                        n_active -=1;  
                        println!("{} folded",player.name);
                        action[street].push(Action::Fold);
                    },
                    Action::Call => {
                        callers+=1; 
                        // players old bet was player_bet, now its current_bet
                        pot += current_bet-player_bet; 
                        println!("{} called {}, current_bet: {}, pot: {}",player.name, current_bet-player_bet, current_bet, pot);
                        action[street].push(Action::Call);
                    },
                    Action::Raise(raise) => {
                        // players old bet was player_bet, now its current_bet + raise 
                        // current bet should be incremented by raise 
                        callers = 1;
                        pot += raise + current_bet - player_bet;
                        current_bet += raise;
                        println!("{} raised {}, current_bet: {}, pot: {}",player.name, raise, current_bet, pot);
                        action[street].push(Action::Raise(raise));
                    },
                    Action::AllIn(chips) => {
                        n_all_in_this_street += 1;
                        all_in_player_ids[street].push(player.id);
                        if chips > current_bet - player_bet {
                            callers = 0;
                            current_bet = chips + player_bet; 
                        }
                        pot += chips; 
                        println!("{} went all in for {}, current_bet: {}, pot: {}",player.name, chips, current_bet, pot);
                        action[street].push(Action::AllIn(chips));
                    }
                }
                idx = (idx+1) % n_players; 

                if n_active <=1 {break 'street}
                // println!("agreed: {} ,all in: {}, folded: {}", agreed_players, n_all_in, n_folded);
                // println!(" {} + {} + {} < {} : {}",agreed_players, n_all_in, n_folded, n_players, (agreed_players + n_all_in + n_folded < n_players));
            }
            pots[street]=pot;
            street +=1 ;
            pot = 0; 
            println!("Pots: {:?}",pots);
            if street > 3{ break 'street } 
        }

        self.showdown(community_cards,pots, all_in_player_ids); 

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
        game.showdown(community_cards,vec![500,0,0,0],vec![vec![],vec![],vec![],vec![]]);
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
