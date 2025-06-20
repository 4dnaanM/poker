use crate::deck::{Card,Rank,Suit}; 
use Rank::*; 
use Suit::*;
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

pub struct HandComparator {}
impl HandComparator {
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
        let n = rank_sorted_hand.len(); 
        let mut hand = Vec::new(); 
        hand.push(rank_sorted_hand[0]);
        for idx in 1..n+1 {
            let card = rank_sorted_hand[idx%n];
            let prev = rank_sorted_hand[(idx-1)%n];
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
                let straight_flush = HandComparator::best_straight(&suits[idx]);
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

    pub fn best_hand(mut hand: Vec<Card> ) -> ([Card;5],Hand) {
        // best 5 card hand from 7
        
        hand.sort_by_key(|card| std::cmp::Reverse(card.0));
            
        let flush = HandComparator::best_flush(&hand);
        let straight = HandComparator::best_straight(&hand);
        let combination = HandComparator::best_combination(&hand);

        let mut best_hand = combination; 
        if let Some((v,h)) = flush {
            if h > best_hand.1 {best_hand = (v,h)}
        }
        if let Some((v,h)) = straight {
            if h > best_hand.1 {best_hand = (v,h)}
        }

        return best_hand;
    }

    pub fn compare_hand(pro_7: Vec<Card>, opp_7: Vec<Card> ) -> i8 {
        assert!(pro_7.len() == 7 && opp_7.len() == 7);
        let pro = HandComparator::best_hand(pro_7);
        let opp = HandComparator::best_hand(opp_7);

        if pro.1 > opp.1 {
            return 1; 
        }
        else if pro.1 < opp.1 {
            return -1; 
        }
        
        for idx in 0..5 {
            if pro.0[idx].0 > opp.0[idx].0 {
                return 1; 
            }
            else if pro.0[idx].0 < opp.0[idx].0 {
                return -1;
            }
        }
        return 0; 
    }

}

mod tests {
    use crate::utils::*;
    use crate::deck::*; 
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
        let flush = HandComparator::best_flush(&hand);
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
        let flush = HandComparator::best_flush(&hand);
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
        let flush = HandComparator::best_flush(&hand);
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
        let flush = HandComparator::best_flush(&hand);
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
        let straight = HandComparator::best_straight(&hand);
        assert!(straight == Some(([Card(Jack,Hearts),Card(Ten,Spades),Card(Nine,Spades),Card(Eight,Spades),Card(Seven,Hearts)],Straight)))
    }

    #[test]
    fn test_ace_straights() {
        let mut hand = vec!(
            Card(Ace,Spades),
            Card(Nine,Spades),
            Card(Three,Diamonds),
            Card(Two,Spades),
            Card(Five,Hearts),
            Card(Four,Spades),
            Card(Jack,Hearts)
        );

        hand.sort_by_key(|card| std::cmp::Reverse(card.0));
        let straight = HandComparator::best_straight(&hand);
        assert!(straight == Some(([Card(Five,Hearts),Card(Four,Spades),Card(Three,Diamonds),Card(Two,Spades),Card(Ace,Spades)],Straight)));

        hand = vec!(
            Card(Ace,Spades),
            Card(Ten,Spades),
            Card(Queen,Diamonds),
            Card(King,Spades),
            Card(Five,Hearts),
            Card(Four,Spades),
            Card(Jack,Hearts)
        );

        hand.sort_by_key(|card| std::cmp::Reverse(card.0));
        let straight = HandComparator::best_straight(&hand);
        assert!(straight == Some(([Card(Ace,Spades),Card(King,Spades),Card(Queen,Diamonds),Card(Jack,Hearts),Card(Ten,Spades)],Straight)));
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
        let straight = HandComparator::best_straight(&hand);
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
        let straight = HandComparator::best_straight(&hand);
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
        let combination = HandComparator::best_combination(&hand);
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
        let combination = HandComparator::best_combination(&hand);
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
        let combination = HandComparator::best_combination(&hand);
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
        let combination = HandComparator::best_combination(&hand);
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
        let combination = HandComparator::best_combination(&hand);
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
        let combination = HandComparator::best_combination(&hand);
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
            let best_hand = HandComparator::best_hand(hand);
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

