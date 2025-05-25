use crate::deck::{Deck, Card, Suit::*, Rank::*};
use crate::player::{Player,Action,PlayerState};
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum Hand {
    RoyalFlush, 
    StraightFlush, 
    Quads, 
    FullHouse, 
    Flush,
    Straight, 
    Trips, 
    TwoPair, 
    Pair, 
    HighCard
}
static HAND_ORDER: &[Hand] = &[
    Hand::RoyalFlush, 
    Hand::StraightFlush, 
    Hand::Quads, 
    Hand::FullHouse, 
    Hand::Flush,
    Hand::Straight, 
    Hand::Trips, 
    Hand::TwoPair, 
    Hand::Pair, 
    Hand::HighCard
];
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        HAND_ORDER.iter().position(|&r| r == *other).unwrap()
            .cmp(&HAND_ORDER.iter().position(|&r| r == *self).unwrap())
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
use Hand::*;

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

    fn best_combination(rank_sorted_hand: &Vec<Card>) -> ([Card;5],Hand) {
        debug_assert!(rank_sorted_hand.windows(2).all(|w| w[0].0 >= w[1].0), "Hand must be sorted in decreasing order by rank");
        
        let mut ranks: Vec<Vec<Card>> = vec![Vec::new();13];
        for card in rank_sorted_hand {
            // card.0 as usize is index sorted by rank
            ranks[12-card.0 as usize].push(*card);
        }
        // until we have 5 cards, return the max len then the next max len then the next etc
        // break ties on rank 
        // sort by key does not change the ordering of equal elements so this works
        ranks.sort_by_key(|vec| std::cmp::Reverse(vec.len()) );

        let mut hand = Vec::new();
        let mut added_cards = 0; 
        let mut idx = 0; 
        let mut hand_name = HighCard;
        while added_cards < 5 {
            if ranks[idx].len() == 2 {
                if hand_name==HighCard {hand_name = Pair;}
                else if hand_name==Pair {hand_name = TwoPair;}
                else if hand_name==Trips {hand_name = FullHouse;}
            } 
            else if ranks[idx].len() == 3 {
                if hand_name==HighCard {hand_name = Trips;}
                else if hand_name==Trips {hand_name = FullHouse;}
            }
            else if ranks[idx].len() == 4 {
                if hand_name==HighCard {hand_name = Quads;}
            } 
            added_cards += ranks[idx].len();

            hand.extend_from_slice(&ranks[idx]);          
            idx+=1; 
        }
        
        return (hand[..5].try_into().unwrap(),hand_name);
    }
    
    fn best_straight(rank_sorted_hand: &Vec<Card>) -> Option<([Card;5],Hand)> {
        // must be sorted in decreasing order
        debug_assert!(rank_sorted_hand.windows(2).all(|w| w[0].0 >= w[1].0), "Hand must be sorted in decreasing order by rank");
        
        // ignoring straight flushes - those are accounted for in best_flush
        let mut hand = Vec::new(); 
        hand.push(rank_sorted_hand[0]);
        for idx in 1..rank_sorted_hand.len() {
            let card = rank_sorted_hand[idx];
            let prev = rank_sorted_hand[idx-1];
            if card.0 + 1 != Some(prev.0) {
                hand.clear(); 
            }
            hand.push(card);
            if hand.len() == 5 {return Some((hand.try_into().unwrap(),Straight))}
        }
        return None;
    }
    
    fn best_flush(rank_sorted_hand: &Vec<Card>) -> Option<([Card;5],Hand)> {
        // sorted in decreasing order
        Deck::print_cards(rank_sorted_hand);
        debug_assert!(rank_sorted_hand.windows(2).all(|w| w[0].0 >= w[1].0), "Hand must be sorted in decreasing order by rank");

        let mut suits: Vec<Vec<Card>> = vec![Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        for card in rank_sorted_hand {
            match card {
                Card(_,Clubs) => {
                    suits[0].push(*card);
                }
                Card(_,Diamonds) => {
                    suits[1].push(*card);
                }
                Card(_,Hearts) => {
                    suits[2].push(*card);
                }
                Card(_,Spades) => {
                    suits[3].push(*card);
                }
            }
        }
        for idx in 0..4{
            if suits[idx].len() >= 5 {
                suits[idx].sort_by_key(|c| std::cmp::Reverse(c.0));
                let straight_flush = Game::best_straight(&suits[idx]);
                match straight_flush {
                    Some((vec,_)) => {
                        match vec[0] {
                            Card(Ace,_) => {return Some((vec,RoyalFlush));},
                            _ => {return Some((vec,StraightFlush));}
                        }
                    },
                    _ => {return Some((suits[idx][..5].try_into().unwrap(),Flush));}
                }
            }    
        }
        
        return None;
    }

    fn best_hand(mut hand: Vec<Card> ) -> ([Card;5],Hand) {
        hand.sort_by_key(|card| std::cmp::Reverse(card.0));
            
        let flush = Game::best_flush(&hand);
        let straight = Game::best_straight(&hand);
        let combination = Game::best_combination(&hand);

        let mut best_hand = combination; 
        if let Some((v,h)) = flush {
            if h > best_hand.1 {best_hand = (v,h)}
        }
        if let Some((v,h)) = straight {
            if h > best_hand.1 {best_hand = (v,h)}
        }

        return best_hand;
    }
    
    fn find_winner(&self, community_cards: [Card;5]) -> Vec<(&Player,([Card;5],Hand))> {

        let mut winners: Vec::<(&Player,([Card;5],Hand))> = Vec::new();
        for player in &self.players {

            if player.state== PlayerState::Folded { 
                continue;
            }
            
            let mut hand: Vec<Card> = Vec::new();
            hand.extend_from_slice(&community_cards);
            hand.extend_from_slice(&player.hand);

            let best_hand = Game::best_hand(hand);

            // now that we have best hand of this person, compare with earlier best hand and replace; 
            if !winners.is_empty() {
                let (_, winner_hand) = winners[0];
                if best_hand.1 > winner_hand.1 {
                    winners.clear(); 
                    winners.push((player,best_hand));
                }
                else if best_hand.1 == winner_hand.1 {
                    // they have same 
                    // look through sorted cards and find first diff
                    let old = winners[0].1.0;
                    let new = best_hand.0;
                    for idx in 0..5 {
                        if new[idx].0 > old[idx].0 {
                            winners.clear(); 
                            winners.push((player,best_hand));
                            break; 
                        }
                        else if old[idx].0 > new[idx].0 {
                            break; 
                        }
                        else if idx == 4 {
                            winners.push((player,best_hand));
                        }
                    }
                }
            } else {winners.push((player,best_hand));}
        }
        println!("Winning Hand: ");
        Deck::print_cards(&winners[0].1.0);
        
        return winners; 
    }
    
    fn showdown(&mut self, community_cards: [Card;5]) -> usize {
        println!("Showdown");
        self.find_winner(community_cards);

        self.players.retain(|player| player.chips > 0);
        for player in self.players.iter_mut() {
            player.reset();
        }
        return 0;
    }

    pub fn play_round(&mut self, dealer: usize){
        
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
                
                if n_all_in + n_folded == n_players - 1 {break 'street;}
    
                let player = &mut self.players[(idx+dealer)%n_players];
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

    #[test]
    fn test_no_flush() {
        let mut hand = vec!(
            Card(Ace,Spades),
            Card(Two,Hearts),
            Card(Three,Diamonds),
            Card(Six,Spades),
            Card(Four,Spades),
            Card(Eight,Clubs),
            Card(King,Spades)
        );

        hand.sort_by_key(|x| std::cmp::Reverse(x.0));
        let flush = Game::best_flush(&hand);
        assert!(flush.is_none());
    }
    
    #[test]
    fn test_flush() {
        let mut hand = vec!(
            Card(Ace,Spades),
            Card(Two,Spades),
            Card(Three,Spades),
            Card(Six,Spades),
            Card(Four,Spades),
            Card(Eight,Spades),
            Card(King,Spades)
        );

        hand.sort_by_key(|x| std::cmp::Reverse(x.0));
        let flush = Game::best_flush(&hand);
        assert!(flush.unwrap() == ([Card(Ace,Spades),Card(King,Spades),Card(Eight,Spades),Card(Six,Spades),Card(Four,Spades)],Flush));
    }

    #[test]
    fn test_straight_flush() {
        let mut hand = vec!(
            Card(Ace,Spades),
            Card(Two,Spades),
            Card(Three,Spades),
            Card(Six,Spades),
            Card(Four,Spades),
            Card(Eight,Hearts),
            Card(Five,Spades)
        );

        hand.sort_by_key(|x| std::cmp::Reverse(x.0));
        let flush = Game::best_flush(&hand);
        assert!(flush.unwrap() == ([Card(Six,Spades),Card(Five,Spades),Card(Four,Spades),Card(Three,Spades),Card(Two,Spades)],StraightFlush));
    }

    #[test]
    fn test_wraparound_not_straight_flush() {

        let mut hand = vec!(
            Card(Ace,Spades),
            Card(Two,Spades),
            Card(Three,Spades),
            Card(Six,Hearts),
            Card(Four,Spades),
            Card(Eight,Hearts),
            Card(King,Spades)
        );

        hand.sort_by_key(|x| std::cmp::Reverse(x.0));
        let flush = Game::best_flush(&hand);
        assert!(flush.unwrap() == ([Card(Ace,Spades),Card(King,Spades),Card(Four,Spades),Card(Three,Spades),Card(Two,Spades)],Flush));

    }

    #[test]
    fn test_straight() {
        let mut hand = vec!(
            Card(King,Spades),
            Card(Nine,Spades),
            Card(Three,Diamonds),
            Card(Ten,Spades),
            Card(Seven,Hearts),
            Card(Eight,Spades),
            Card(Jack,Hearts)
        );

        hand.sort_by_key(|card| std::cmp::Reverse(card.0));
        let straight = Game::best_straight(&hand);
        assert!(straight == Some(([Card(Jack,Hearts),Card(Ten,Spades),Card(Nine,Spades),Card(Eight,Spades),Card(Seven,Hearts)],Straight)))
    }

    #[test]
    fn test_straight_higher() {
        let mut hand = vec!(
            Card(Five,Diamonds),
            Card(Nine,Clubs),
            Card(Two,Spades),
            Card(Eight,Hearts),
            Card(Ten,Hearts),
            Card(Six,Hearts),
            Card(Seven,Diamonds)
        );

        hand.sort_by_key(|card| std::cmp::Reverse(card.0));
        let straight = Game::best_straight(&hand);
        assert!(straight == Some(([Card(Ten,Hearts),Card(Nine,Clubs),Card(Eight,Hearts),Card(Seven,Diamonds),Card(Six,Hearts)],Straight)))
    }


    #[test]
    fn test_no_wraparound_straight() {
        let mut hand = vec!(
            Card(Ace,Spades),
            Card(Two,Hearts),
            Card(Three,Spades),
            Card(King,Diamonds),
            Card(Four,Clubs),
            Card(Seven,Hearts),
            Card(King,Spades)
        );

        hand.sort_by_key(|x| std::cmp::Reverse(x.0));
        let straight = Game::best_straight(&hand);
        assert!(straight.is_none());
    }

    #[test]
    fn test_high_card() {
        let mut hand = vec!(
            Card(King,Spades),
            Card(Nine,Spades),
            Card(Three,Diamonds),
            Card(Ten,Spades),
            Card(Seven,Hearts),
            Card(Jack,Spades),
            Card(Five,Hearts)
        );

        hand.sort_by_key(|x| std::cmp::Reverse(x.0));
        let combination = Game::best_combination(&hand);
        assert!(combination == ([Card(King,Spades),Card(Jack,Spades),Card(Ten,Spades),Card(Nine,Spades),Card(Seven,Hearts)],HighCard))
    }

    #[test]
    fn test_pair() {
        let mut hand = vec!(
            Card(King,Spades),
            Card(Nine,Spades),
            Card(Three,Diamonds),
            Card(Ten,Spades),
            Card(Seven,Hearts),
            Card(Eight,Hearts),
            Card(King,Diamonds)
        );

        hand.sort_by_key(|x| std::cmp::Reverse(x.0));
        let combination = Game::best_combination(&hand);
        assert!(combination == ([Card(King,Spades),Card(King,Diamonds),Card(Ten,Spades),Card(Nine,Spades),Card(Eight,Hearts)],Pair))
    }

    #[test]
    fn test_trips() {
        let mut hand = vec!(
            Card(King,Spades),
            Card(Nine,Spades),
            Card(Three,Diamonds),
            Card(Ten,Spades),
            Card(Seven,Hearts),
            Card(Seven,Clubs),
            Card(Seven,Diamonds)
        );

        hand.sort_by_key(|x| std::cmp::Reverse(x.0));
        let combination = Game::best_combination(&hand);
        assert!(combination == ([Card(Seven,Hearts),Card(Seven,Clubs),Card(Seven,Diamonds),Card(King,Spades),Card(Ten,Spades)],Trips))
    }

    #[test]
    fn test_two_pair() {
        let mut hand = vec!(
            Card(King,Spades),
            Card(Nine,Spades),
            Card(Three,Diamonds),
            Card(Ten,Spades),
            Card(Seven,Hearts),
            Card(Ten,Diamonds),
            Card(King,Clubs)
        );

        hand.sort_by_key(|x| std::cmp::Reverse(x.0));
        let combination = Game::best_combination(&hand);
        assert!(combination == ([Card(King,Spades),Card(King,Clubs),Card(Ten,Spades),Card(Ten,Diamonds),Card(Nine,Spades)],TwoPair))
    }

    #[test]
    fn test_full_house() {
        let mut hand = vec!(
            Card(King,Spades),
            Card(King,Hearts),
            Card(Three,Diamonds),
            Card(Ten,Spades),
            Card(Seven,Hearts),
            Card(Three,Spades),
            Card(King,Clubs)
        );

        hand.sort_by_key(|x| std::cmp::Reverse(x.0));
        let combination = Game::best_combination(&hand);
        assert!(combination == ([Card(King,Spades),Card(King,Hearts),Card(King,Clubs),Card(Three,Diamonds),Card(Three,Spades)],FullHouse))

    }

    #[test]
    fn test_quads() {
        let mut hand = vec!(
            Card(King,Spades),
            Card(King,Hearts),
            Card(Three,Diamonds),
            Card(Ten,Spades),
            Card(Seven,Hearts),
            Card(King,Diamonds),
            Card(King,Clubs)
        );

        hand.sort_by_key(|x| std::cmp::Reverse(x.0));
        let combination = Game::best_combination(&hand);
        assert!(combination == ([Card(King,Spades),Card(King,Hearts),Card(King,Diamonds),Card(King,Clubs),Card(Ten,Spades)],Quads))

    }

    // for looking at random tests because its cool 
    // #[test]
    fn test_loop() {
        loop {

            let mut d = Deck::new(); 
            let hand  = vec![
                d.deal().unwrap(), 
                d.deal().unwrap(), 
                d.deal().unwrap(), 
                d.deal().unwrap(), 
                d.deal().unwrap(), 
                d.deal().unwrap(), 
                d.deal().unwrap()
            ];
            Deck::print_cards(&hand);
            let best_hand = Game::best_hand(hand);
            Deck::print_cards(&best_hand.0);

            let mut input = String::new();
            println!("Press Enter to continue or type 'exit' to quit: ");
            std::io::stdin().read_line(&mut input).unwrap();
            if input.trim().eq_ignore_ascii_case("exit") {
                break;
            }

        }
    }
}
