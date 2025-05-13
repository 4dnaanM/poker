use rand::Rng;
use std::cmp::Ordering;

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}
impl From<Suit> for String {
    fn from(suit: Suit) -> Self {
        match suit {
            Suit::Hearts => '♥'.to_string(),
            Suit::Diamonds => '♦'.to_string(),
            Suit::Clubs => '♣'.to_string(),
            Suit::Spades => '♠'.to_string(),
        }
    }
}
impl From<String> for Suit {
    fn from(string: String) -> Suit {
        match string.as_str() {
            "♥" => Suit::Hearts,
            "♦" => Suit::Diamonds,
            "♣" => Suit::Clubs,
            "♠" => Suit::Spades,
            _ => panic!("Invalid value for suit: '{}'", string),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
static RANK_ORDER: &[Rank] = &[
    Rank::Two,
    Rank::Three,
    Rank::Four,
    Rank::Five,
    Rank::Six,
    Rank::Seven,
    Rank::Eight,
    Rank::Nine,
    Rank::Ten,
    Rank::Jack,
    Rank::Queen,
    Rank::King,
    Rank::Ace,
];
impl From<Rank> for String {
    fn from(rank: Rank) -> String {
        match rank {
            Rank::Two => "2".to_string(),
            Rank::Three => "3".to_string(),
            Rank::Four => "4".to_string(),
            Rank::Five => "5".to_string(),
            Rank::Six => "6".to_string(),
            Rank::Seven => "7".to_string(),
            Rank::Eight => "8".to_string(),
            Rank::Nine => "9".to_string(),
            Rank::Ten => "10".to_string(),
            Rank::Jack => "J".to_string(),
            Rank::Queen => "Q".to_string(),
            Rank::King => "K".to_string(),
            Rank::Ace => "A".to_string(),
        }
    }
}
impl From<String> for Rank {
    fn from(string: String) -> Rank {
        match string.as_str() {
            "2" => Rank::Two,
            "3" => Rank::Three,
            "4" => Rank::Four,
            "5" => Rank::Five,
            "6" => Rank::Six,
            "7" => Rank::Seven,
            "8" => Rank::Eight,
            "9" => Rank::Nine,
            "10" => Rank::Ten,
            "J" => Rank::Jack,
            "Q" => Rank::Queen,
            "K" => Rank::King,
            "A" => Rank::Ace,
            _ => panic!("Invalid value for rank: '{}'", string),
        }
    }
}
impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Rank {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_index = RANK_ORDER.iter().position(|&r| r == *self).unwrap();
        let other_index = RANK_ORDER.iter().position(|&r| r == *other).unwrap();
        self_index.cmp(&other_index)
    }
}

impl std::ops::Add<u8> for Rank {
    type Output = Option<Rank>;

    fn add(self, rhs: u8) -> Option<Rank> {
        let current_index = RANK_ORDER.iter().position(|&r| r == self).unwrap();
        if current_index + rhs as usize >= 13  {return None}
        Some(*RANK_ORDER.get(current_index + rhs as usize).unwrap())
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct Card(pub Rank, pub Suit);
impl Card {
    pub fn get_display_lines(&self) -> [String;8] {
        let first_line = format!("_________");
        let (second_line, third_line,fourth_line, fifth_line, sixth_line, seventh_line, eighth_line) = match self.0 {
            Rank::Ace => (
                format!("|{}      |",String::from(self.0)),
                format!("|       |"),
                format!("|       |"),
                format!("|   {}   |",String::from(self.1)),
                format!("|       |"),
                format!("|       |"),
                format!("|______{}|",String::from(self.0))
            ),
            Rank::Two => (
                format!("|{}      |",String::from(self.0)),
                format!("|   {}   |",String::from(self.1)),
                format!("|       |"),
                format!("|       |"),
                format!("|       |"),
                format!("|   {}   |",String::from(self.1)),
                format!("|______{}|",String::from(self.0))
            ),
            Rank::Three => (
                format!("|{}      |",String::from(self.0)),
                format!("|   {}   |",String::from(self.1)),
                format!("|       |"),
                format!("|   {}   |",String::from(self.1)),
                format!("|       |"),
                format!("|   {}   |",String::from(self.1)),
                format!("|______{}|",String::from(self.0))
            ),
            Rank::Four => (
                format!("|{}      |",String::from(self.0)),
                format!("| {}   {} |",String::from(self.1), String::from(self.1)),
                format!("|       |"),
                format!("|       |"),
                format!("|       |"),
                format!("| {}   {} |",String::from(self.1), String::from(self.1)),
                format!("|______{}|",String::from(self.0))
            ),
            Rank::Five => (
                format!("|{}      |",String::from(self.0)),
                format!("| {}   {} |",String::from(self.1), String::from(self.1)),
                format!("|       |"),
                format!("|   {}   |",String::from(self.1)),
                format!("|       |"),
                format!("| {}   {} |",String::from(self.1), String::from(self.1)),
                format!("|______{}|",String::from(self.0))
            ),
            Rank::Six => (
                format!("|{}      |",String::from(self.0)),
                format!("| {}   {} |",String::from(self.1), String::from(self.1)),
                format!("|       |"),
                format!("| {}   {} |",String::from(self.1), String::from(self.1)),
                format!("|       |"),
                format!("| {}   {} |",String::from(self.1), String::from(self.1)),
                format!("|______{}|",String::from(self.0))
            ),
            Rank::Seven => (
                format!("|{}      |",String::from(self.0)),
                format!("| {}   {} |",String::from(self.1), String::from(self.1)),
                format!("|   {}   |",String::from(self.1)),
                format!("| {}   {} |",String::from(self.1), String::from(self.1)),
                format!("|       |"),
                format!("| {}   {} |",String::from(self.1), String::from(self.1)),
                format!("|______{}|",String::from(self.0))
            ),
            Rank::Eight => (
                format!("|{}      |",String::from(self.0)),
                format!("| {}   {} |",String::from(self.1), String::from(self.1)),
                format!("|   {}   |",String::from(self.1)),
                format!("| {}   {} |",String::from(self.1), String::from(self.1)),
                format!("|   {}   |",String::from(self.1)),
                format!("| {}   {} |",String::from(self.1), String::from(self.1)),
                format!("|______{}|",String::from(self.0))
            ),
            Rank::Nine => (
                format!("|{}      |",String::from(self.0)),
                format!("| {}   {} |",String::from(self.1), String::from(self.1)),
                format!("|   {}   |",String::from(self.1)),
                format!("| {} {} {} |",String::from(self.1), String::from(self.1),String::from(self.1)),
                format!("|   {}   |",String::from(self.1)),
                format!("| {}   {} |",String::from(self.1), String::from(self.1)),
                format!("|______{}|",String::from(self.0))
            ),
            Rank::Ten => (
                format!("|{}     |",String::from(self.0)),
                format!("| {}   {} |",String::from(self.1), String::from(self.1)),
                format!("| {}   {} |",String::from(self.1), String::from(self.1)),
                format!("| {}   {} |",String::from(self.1), String::from(self.1)),
                format!("| {}   {} |",String::from(self.1), String::from(self.1)),
                format!("| {}   {} |",String::from(self.1), String::from(self.1)),
                format!("|_____{}|",String::from(self.0))
            ),
            Rank::Jack => (
                format!("|{}      |",String::from(self.0)),
                format!("| {}{}{}{}{} |",String::from(self.1),String::from(self.1),String::from(self.1),String::from(self.1),String::from(self.1)),
                format!("| {}{}{}{}{} |",String::from(self.1),String::from(self.1),String::from(self.1),String::from(self.1),String::from(self.1)),
                format!("| {}{}{}{}{} |",String::from(self.1),String::from(self.1),String::from(self.1),String::from(self.1),String::from(self.1)),
                format!("| {}{}{}{}{} |",String::from(self.1),String::from(self.1),String::from(self.1),String::from(self.1),String::from(self.1)),
                format!("| {}{}{}{}{} |",String::from(self.1),String::from(self.1),String::from(self.1),String::from(self.1),String::from(self.1)),
                format!("|______{}|",String::from(self.0))
            ),
            Rank::Queen => (
                format!("|{}      |",String::from(self.0)),
                format!("| {}{}{}{}{} |",String::from(self.1),String::from(self.1),String::from(self.1),String::from(self.1),String::from(self.1)),
                format!("| {}{}{}{}{} |",String::from(self.1),String::from(self.1),String::from(self.1),String::from(self.1),String::from(self.1)),
                format!("| {}{}{}{}{} |",String::from(self.1),String::from(self.1),String::from(self.1),String::from(self.1),String::from(self.1)),
                format!("| {}{}{}{}{} |",String::from(self.1),String::from(self.1),String::from(self.1),String::from(self.1),String::from(self.1)),
                format!("| {}{}{}{}{} |",String::from(self.1),String::from(self.1),String::from(self.1),String::from(self.1),String::from(self.1)),
                format!("|______{}|",String::from(self.0))
            ),
            Rank::King => (
                // format!("|{}      |",String::from(self.0)),
                // format!("|       |"),
                // format!("|       |"),
                // format!("|   {}   |",String::from(self.1)),
                // format!("|       |"),
                // format!("|       |"),
                // format!("|______{}|",String::from(self.0))
                format!("|{}      |",String::from(self.0)),
                format!("| {}{}{}{}{} |",String::from(self.1),String::from(self.1),String::from(self.1),String::from(self.1),String::from(self.1)),
                format!("| {}{}{}{}{} |",String::from(self.1),String::from(self.1),String::from(self.1),String::from(self.1),String::from(self.1)),
                format!("| {}{}{}{}{} |",String::from(self.1),String::from(self.1),String::from(self.1),String::from(self.1),String::from(self.1)),
                format!("| {}{}{}{}{} |",String::from(self.1),String::from(self.1),String::from(self.1),String::from(self.1),String::from(self.1)),
                format!("| {}{}{}{}{} |",String::from(self.1),String::from(self.1),String::from(self.1),String::from(self.1),String::from(self.1)),
                format!("|______{}|",String::from(self.0))
            )
        };

        [
            first_line, 
            second_line, 
            third_line, 
            fourth_line, 
            fifth_line, 
            sixth_line,
            seventh_line,
            eighth_line
        ]
    }
}

#[derive(Debug)]
pub struct Deck {
    pub deck: Vec<Card>,
}
impl Deck {
    pub fn new() -> Deck {
        let mut deck = Vec::new();
        for suit in vec![Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for rank in RANK_ORDER {
                deck.push(Card(*rank, suit));
            }
        }
        Deck { deck }
    }

    pub fn deal(&mut self) -> Option<Card> {
        if self.deck.len() == 0 {
            return None;
        }
        let index = rand::thread_rng().gen_range(0..self.deck.len());
        let card = self.deck.swap_remove(index);
        Some(card)
    }

    pub fn print_cards<T: AsRef<[Card]>>(cards: T) {
        let mut lines = vec![String::new(); 8];
        let mut count_cards = 0; 
        for card in cards.as_ref() {
            let card_lines = card.get_display_lines();
            for line_number in 0..8 {
                lines[line_number].push_str(&format!("{} ", card_lines[line_number]));
            }
            count_cards += 1; 
            if count_cards==13 {
                count_cards=0; 
                println!("{}", lines.join("\n"));    
                lines = vec![String::new(); 8];
            }
        }
        if count_cards>0 {
            println!("{}", lines.join("\n"));    
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suit_conversion() {
        assert_eq!(String::from(Suit::Hearts), "♥");
        assert_eq!(String::from(Suit::Diamonds), "♦");
        assert_eq!(String::from(Suit::Clubs), "♣");
        assert_eq!(String::from(Suit::Spades), "♠");

        assert_eq!(Suit::from("♥".to_string()), Suit::Hearts);
        assert_eq!(Suit::from("♦".to_string()), Suit::Diamonds);
        assert_eq!(Suit::from("♣".to_string()), Suit::Clubs);
        assert_eq!(Suit::from("♠".to_string()), Suit::Spades);
    }

    #[test]
    fn test_rank_conversion() {
        assert_eq!(String::from(Rank::Two), "2");
        assert_eq!(String::from(Rank::Ace), "A");
        assert_eq!(Rank::from("2".to_string()), Rank::Two);
        assert_eq!(Rank::from("A".to_string()), Rank::Ace);
    }

    #[test]
    fn test_deck_creation() {
        let deck = Deck::new();
        assert_eq!(deck.deck.len(), 52);
    }

    #[test]
    fn test_deal_card() {
        let mut deck = Deck::new();
        let initial_len = deck.deck.len();
        let card = deck.deal();
        assert!(card.is_some());
        assert_eq!(deck.deck.len(), initial_len - 1);
    }

    #[test]
    fn test_deal_empty_deck() {
        let mut deck = Deck { deck: vec![] };
        let card = deck.deal();
        assert!(card.is_none());
    }

    #[test]
    fn test_card_display_lines() {
        let card = Card(Rank::Ace, Suit::Hearts);
        let lines = card.get_display_lines();
        assert_eq!(lines[0], "_________");
        assert_eq!(lines[4], "|   ♥   |");
    }
}
