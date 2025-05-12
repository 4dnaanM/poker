use crate::deck::{Card,Deck};

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
    Bet(u32),
    AllIn(u32)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Player {
    pub name: String, 
    pub chips: u32,
    pub hand: Vec<Card>,
    pub state: PlayerState,
    pub bet: u32
}

impl Player {
    pub fn new(name: String, chips: u32) -> Player {
        Player { name, chips, hand: Vec::new(), state: PlayerState::Active , bet: 0}
    }
    pub fn act(&mut self, pot: u32, board: &[Card], bet: u32, action:&Vec<Vec<Action>>) -> Action {
        if self.bet < bet {
            if self.chips > bet - self.bet {
                return Action::Call;
            }
            return self.go_all_in(); 
        }
        return Action::Check;
    }

    pub fn display(&self) {
        println!("{}: Stack: {}, Bet: {}, State: {:?}",self.name, self.chips, self.bet, self.state);
        Deck::print_cards(&[self.hand[0],self.hand[1]]);
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

    pub fn bet_blind(&mut self, blind: u32) {
        if self.chips < blind {
            self.go_all_in(); 
            return; 
        }
        self.bet += blind;
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
        let player = Player::new("Alice".to_string(), 1000);
        assert_eq!(player.name, "Alice");
        assert_eq!(player.chips, 1000);
        assert!(player.hand.is_empty());
        assert_eq!(player.state, PlayerState::Active);
    }

    #[test]
    fn test_player_deal_card() {
        let mut player = Player::new("Bob".to_string(), 500);
        let mut d = Deck::new(); 
        let card = d.deal().unwrap(); 
        player.deal_card(card.clone());
        assert_eq!(player.hand.len(), 1);
        assert_eq!(player.hand[0], card);
    }

    #[test]
    #[should_panic]
    fn test_player_deal_card_panic() {
        let mut player = Player::new("Charlie".to_string(), 300);
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
        let mut player = Player::new("Dana".to_string(), 200);
        let action = player.go_all_in();
        assert_eq!(action, Action::AllIn(200));
        assert_eq!(player.chips, 0);
        assert_eq!(player.state, PlayerState::AllIn);
    }

    #[test]
    fn test_player_bet_blind() {
        let mut player = Player::new("Eve".to_string(), 100);
        player.bet_blind(50);
        assert_eq!(player.chips, 50);
        assert_eq!(player.state, PlayerState::Active);
    }

    #[test]
    fn test_player_bet_blind_all_in() {
        let mut player = Player::new("Frank".to_string(), 30);
        player.bet_blind(50);
        assert_eq!(player.chips, 0);
        assert_eq!(player.state, PlayerState::AllIn);
    }
}
