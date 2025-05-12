use crate::deck::Card;

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
    pub state: PlayerState
}

impl Player {
    pub fn new(name: String, chips: u32) -> Player {
        Player { name, chips, hand: Vec::new(), state: PlayerState::Active }
    }
    pub fn act(&mut self, pot: u32, board: &[Card], bet: u32, action:&Vec<Vec<Action>>) -> Action {
        // this is how you gamble
        return self.go_all_in();
    }
    pub fn deal_card(&mut self, card: Card) {
        assert!(self.hand.len()<2);
        self.hand.push(card); 
    }

    pub fn go_all_in(&mut self) -> Action {
        let chips = self.chips; 
        self.chips = 0; 
        self.state = PlayerState::AllIn;
        return Action::AllIn(chips);
    }

    pub fn bet_blind(&mut self, blind: u32) {
        if self.chips < blind {
            self.go_all_in(); 
            return; 
        }
        self.chips -= blind; 
    }
}

