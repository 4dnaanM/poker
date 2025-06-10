use crate::deck::{Card,Deck};
use rand::Rng;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum PlayerState {
    Active, 
    Folded, 
    AllIn
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Action { 
    Fold, 
    Call, 
    Check,
    Raise(u32),
    AllIn(u32)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Player {
    pub id: usize,
    pub name: String, 
    pub chips: u32,
    pub hand: Vec<Card>,
    pub state: PlayerState,
    pub bet: u32
}

impl Player {

    pub fn new(id: usize, name: String, chips: u32) -> Player {
        Player { id, name, chips, hand: Vec::new(), state: PlayerState::Active , bet: 0}
    }
    
    pub fn act(&mut self, pot: u32, board: &[Card], to_call: u32, action:&Vec<Vec<Action>>) -> Action {
        
        let mut rng = rand::thread_rng();

        // If there's something to call
        if to_call > 0 {
            if self.chips > to_call {
                // Randomly decide between fold, call, or raise
                let choice = rng.gen_range(0..100);
                if choice < 40 {
                    // 40% chance to call
                    return self.call(to_call);
                } else if choice < 70 {
                    // 30% chance to raise
                    let max_raise = self.chips - to_call;
                    if max_raise > 0 {
                        let raise_amount = rng.gen_range(1..=max_raise.min(pot / 2).max(1));
                        return self.raise(to_call, raise_amount);
                    } else {
                        return self.call(to_call);
                    }
                } else if choice < 90 {
                    // 20% chance to fold
                    return self.fold();
                } else {
                    // 10% chance to go all-in
                    return self.go_all_in();
                }
            } else {
                // Not enough chips to call, decide between all-in or fold
                if rng.gen_bool(0.7) {
                    return self.go_all_in();
                } else {
                    return self.fold();
                }
            }
        }

        // No bet to call, decide between check, bet, or all-in
        if self.chips > 0 {
            let choice = rng.gen_range(0..100);
            if choice < 60 {
                // 60% chance to check
                return Action::Check;
            } else if choice < 90 {
                // 30% chance to bet/raise
                let raise_amount = rng.gen_range(1..=self.chips.min(pot / 2).max(1));
                return self.raise(0, raise_amount);
            } else {
                // 10% chance to go all-in
                return self.go_all_in();
            }
        }

        Action::Check

    }
    
    pub fn display(&self) {
        println!("{}: Stack: {}, Bet: {}, State: {:?}",self.name, self.chips, self.bet, self.state);
        Deck::print_cards(&[self.hand[0],self.hand[1]]);
    }

    pub fn deal_chips(&mut self, chips: u32) {
        println!("{} got {} chips",self.name, chips);
        self.chips += chips; 
    }

    pub fn deal_card(&mut self, card: Card) {
        assert!(self.hand.len()<2);
        self.hand.push(card); 
    }

    pub fn go_all_in(&mut self) -> Action {
        let chips = self.chips; 
        self.chips = 0; 
        self.state = PlayerState::AllIn;
        self.bet += chips;
        return Action::AllIn(chips);
    }

    fn fold(&mut self) -> Action {
        self.state = PlayerState::Folded;
        return Action::Fold;
    }

    fn raise(&mut self, call_amount: u32, raise_amount: u32) -> Action {
        self.bet += raise_amount + call_amount;
        self.chips -= raise_amount + call_amount;
        return Action::Raise(raise_amount);
    }

    fn call(&mut self, call_amount: u32) -> Action {
        self.bet += call_amount;
        self.chips -= call_amount;
        return Action::Call;
    }

    pub fn bet_blind(&mut self, blind: u32) {
        if self.chips <= blind {
            self.go_all_in(); 
            return; 
        }
        self.bet = blind;
        self.chips -= blind; 
    }
    
    pub fn reset(&mut self) {
        self.hand = Vec::new(); 
        self.state = PlayerState::Active; 
        self.bet = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_new() {
        let player = Player::new(0,"Alice".to_string(), 1000);
        assert_eq!(player.name, "Alice");
        assert_eq!(player.chips, 1000);
        assert!(player.hand.is_empty());
        assert_eq!(player.state, PlayerState::Active);
    }

    #[test]
    fn test_player_deal_card() {
        let mut player = Player::new(0,"Bob".to_string(), 500);
        let mut d = Deck::new(); 
        let card = d.deal().unwrap(); 
        player.deal_card(card.clone());
        assert_eq!(player.hand.len(), 1);
        assert_eq!(player.hand[0], card);
    }

    #[test]
    #[should_panic]
    fn test_player_deal_card_panic() {
        let mut player = Player::new(0,"Charlie".to_string(), 300);
        let mut d = Deck::new();
        let card1 = d.deal().unwrap();
        let card2 = d.deal().unwrap();
        let card3 = d.deal().unwrap();
        player.deal_card(card1);
        player.deal_card(card2);
        player.deal_card(card3); // Should panic
    }

    #[test]
    fn test_player_go_all_in() {
        let mut player = Player::new(0,"Dana".to_string(), 200);
        let action = player.go_all_in();
        assert_eq!(action, Action::AllIn(200));
        assert_eq!(player.chips, 0);
        assert_eq!(player.state, PlayerState::AllIn);
    }

    #[test]
    fn test_player_bet_blind() {
        let mut player = Player::new(0,"Eve".to_string(), 100);
        player.bet_blind(50);
        assert_eq!(player.chips, 50);
        assert_eq!(player.state, PlayerState::Active);
    }

    #[test]
    fn test_player_bet_blind_all_in() {
        let mut player = Player::new(0,"Frank".to_string(), 30);
        player.bet_blind(50);
        assert_eq!(player.chips, 0);
        assert_eq!(player.state, PlayerState::AllIn);
    }
}
