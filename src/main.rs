use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Rank {
    ACE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    JACK,
    QUEEN,
    KING,
}

impl Rank {
    /*
    fn pred(&self) -> Self {
        match self {
            Rank::ACE => Rank::KING,
            Rank::TWO => Rank::ACE,
            Rank::THREE => Rank::TWO,
            Rank::FOUR => Rank::THREE,
            Rank::FIVE => Rank::FOUR,
            Rank::SIX => Rank::FIVE,
            Rank::SEVEN => Rank::SIX,
            Rank::EIGHT => Rank::SEVEN,
            Rank::NINE => Rank::EIGHT,
            Rank::TEN => Rank::NINE,
            Rank::JACK => Rank::TEN,
            Rank::QUEEN => Rank::JACK,
            Rank::KING => Rank::QUEEN,
        }
    }
    */

    fn succ(&self) -> Self {
        match self {
            Rank::ACE => Rank::TWO,
            Rank::TWO => Rank::THREE,
            Rank::THREE => Rank::FOUR,
            Rank::FOUR => Rank::FIVE,
            Rank::FIVE => Rank::SIX,
            Rank::SIX => Rank::SEVEN,
            Rank::SEVEN => Rank::EIGHT,
            Rank::EIGHT => Rank::NINE,
            Rank::NINE => Rank::TEN,
            Rank::TEN => Rank::JACK,
            Rank::JACK => Rank::QUEEN,
            Rank::QUEEN => Rank::KING,
            Rank::KING => Rank::ACE,
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rank::ACE => "A",
                Rank::TWO => "2",
                Rank::THREE => "3",
                Rank::FOUR => "4",
                Rank::FIVE => "5",
                Rank::SIX => "6",
                Rank::SEVEN => "7",
                Rank::EIGHT => "8",
                Rank::NINE => "9",
                Rank::TEN => "T",
                Rank::JACK => "J",
                Rank::QUEEN => "Q",
                Rank::KING => "K",
            }
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Suit {
    CLUB,
    SPADE,
    HEART,
    DIAMOND,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Suit::CLUB => "C",
                Suit::SPADE => "S",
                Suit::HEART => "H",
                Suit::DIAMOND => "D",
            }
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    /*
    fn pred(&self) -> Self {
        Card {
            rank: self.rank.pred(),
            suit: self.suit.clone(),
        }
    }
    */

    fn succ(&self) -> Self {
        Card {
            rank: self.rank.succ(),
            suit: self.suit.clone(),
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

#[derive(PartialEq)]
enum MeldType {
    RUN,
    SET,
}

pub type Hand = Vec<Card>;

fn hand_to_string(hand: &Hand) -> String {
    let mut string = "Hand: [ ".to_string();
    for card in hand {
        string.push_str(format!("{} ", card).as_str());
    }
    string.push_str("]");
    string
}

const MELD_MINIMUM: usize = 3;

/// Creates a hand from an input string
///
/// # Parameters
///
/// - `hand_str`: A space-delimited string of card character pairs. C S H D for
///     Club, Spade, Heart, and Diamond; and A 2 3 4 5 6 7 8 9 T J Q K for card
///     rank. e.g. TD is the 10 of Diamonds.
///
/// # Returns
///
/// - `Some` vector of cards with the corresponding suits and ranks if the input
///     string is formatted correctly, `None` otherwise.
///
/// # Examples
///
/// ```rust
/// let hand = vec![
///     Card {
///         rank: Rank::ACE,
///         suit: Suit::SPADE,
///     },
///     Card {
///         rank: Rank::TEN,
///         suit: Suit::CLUB,
///     },
///     Card {
///         rank: Rank::FIVE,
///         suit: Suit::HEART,
///     }];
/// assert_eq!(hand, hand_from_str("AS TC 5H"));
/// ```
pub fn hand_from_str(hand_str: &str) -> Option<Hand> {
    let mut hand: Hand = Vec::new();
    let hand_opts: Vec<Option<Card>> = hand_str
        .split(' ')
        .map(|str: &str| {
            let rank = match str.chars().nth(0) {
                Some('A') => Some(Rank::ACE),
                Some('2') => Some(Rank::TWO),
                Some('3') => Some(Rank::THREE),
                Some('4') => Some(Rank::FOUR),
                Some('5') => Some(Rank::FIVE),
                Some('6') => Some(Rank::SIX),
                Some('7') => Some(Rank::SEVEN),
                Some('8') => Some(Rank::EIGHT),
                Some('9') => Some(Rank::NINE),
                Some('T') => Some(Rank::TEN),
                Some('J') => Some(Rank::JACK),
                Some('Q') => Some(Rank::QUEEN),
                Some('K') => Some(Rank::KING),
                _ => None,
            };
            let suit = match str.chars().nth(1) {
                Some('C') => Some(Suit::CLUB),
                Some('S') => Some(Suit::SPADE),
                Some('H') => Some(Suit::HEART),
                Some('D') => Some(Suit::DIAMOND),
                _ => None,
            };
            if rank.is_none() || suit.is_none() {
                None
            } else {
                Some(Card {
                    rank: rank.unwrap(),
                    suit: suit.unwrap(),
                })
            }
        })
        .collect();
    for card in hand_opts {
        match card {
            Some(c) => {
                hand.push(c);
            }
            None => {
                return None;
            }
        }
    }
    Some(hand)
}

fn has_gin_helper(card: &Card, meld_length: usize, meld_type: MeldType, hand: &Hand) -> bool {
    if hand.len() == 0 && meld_length >= MELD_MINIMUM {
        return true;
    }
    for i in 0..hand.len() {
        let mut new_hand = hand.to_vec();
        let new_card = new_hand.remove(i);
        if meld_type == MeldType::SET && card.rank == new_card.rank {
            if has_gin_helper(&new_card, meld_length + 1, MeldType::SET, &new_hand) {
                return true;
            }
        } else if meld_type == MeldType::RUN
            && card.succ() == new_card
            && !(card.rank == Rank::ACE && meld_length > 1)
        {
            if has_gin_helper(&new_card, meld_length + 1, MeldType::RUN, &new_hand) {
                return true;
            }
        } else if meld_length >= MELD_MINIMUM {
            if has_gin_helper(&new_card, 1, MeldType::SET, &new_hand) {
                return true;
            }
            if has_gin_helper(&new_card, 1, MeldType::RUN, &new_hand) {
                return true;
            }
        }
    }
    false
}

/// Checks if a hand can gin
///
/// # Parameters
///
/// - `hand`: A vector of cards that we want to check for gin
///
/// # Returns
///
/// - `true` if the hand can gin, `false` otherwise
///
/// # Examples
///
/// ```rust
/// assert!(has_gin(
///     &hand_from_str("7H 8H 9H TH 6H 6D 6S KH KD KC").unwrap()
/// ));
/// ```
///
/// # See also
///
/// - [Gin Rummy](http://rummytalk.com/294-2/)
pub fn has_gin(hand: &Hand) -> bool {
    if hand.len() == 0 {
        return true;
    }
    for i in 0..hand.len() {
        let mut new_hand = hand.to_vec();
        let new_card = new_hand.remove(i);
        if has_gin_helper(&new_card, 1, MeldType::SET, &new_hand) {
            return true;
        }
        if has_gin_helper(&new_card, 1, MeldType::RUN, &new_hand) {
            return true;
        }
    }
    false
}

fn main() {
    let hand = hand_from_str("7H 8H 9H TH 6H 6D 6S KH KD KC").unwrap();
    println!("{}", hand_to_string(&hand));
    println!("Can gin: {}", has_gin(&hand));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_from_str() {
        let hand = vec![
            Card {
                rank: Rank::SEVEN,
                suit: Suit::HEART,
            },
            Card {
                rank: Rank::EIGHT,
                suit: Suit::HEART,
            },
            Card {
                rank: Rank::NINE,
                suit: Suit::HEART,
            },
            Card {
                rank: Rank::TEN,
                suit: Suit::HEART,
            },
            Card {
                rank: Rank::SIX,
                suit: Suit::HEART,
            },
            Card {
                rank: Rank::SIX,
                suit: Suit::DIAMOND,
            },
            Card {
                rank: Rank::SIX,
                suit: Suit::SPADE,
            },
            Card {
                rank: Rank::KING,
                suit: Suit::HEART,
            },
            Card {
                rank: Rank::KING,
                suit: Suit::DIAMOND,
            },
            Card {
                rank: Rank::KING,
                suit: Suit::CLUB,
            },
        ];
        let hand2 = hand_from_str("7H 8H 9H TH 6H 6D 6S KH KD KC").unwrap();
        /*
        zip(&hand, &hand2)
            .map(|(x, y)| x == y)
            .fold(true, |x, y| x && y)
        */
        for i in 0..10 {
            assert_eq!(hand[i], hand2[i]);
        }
    }

    #[test]
    fn test_short_set() {
        assert!(!has_gin(&hand_from_str("AC AH").unwrap()));
    }

    #[test]
    fn test_short_run() {
        assert!(!has_gin(&hand_from_str("AC 2C").unwrap()));
    }

    #[test]
    fn test_set() {
        assert!(has_gin(&hand_from_str("AC AH AD").unwrap()));
    }

    #[test]
    fn test_runs() {
        assert!(has_gin(&hand_from_str("AC 2C 3C").unwrap()));
        assert!(has_gin(&hand_from_str("QH KH AH").unwrap()));
    }

    #[test]
    fn test_long_run() {
        assert!(has_gin(&hand_from_str("5S 6S 7S 8S 9S").unwrap()));
    }

    #[test]
    fn test_long_set() {
        assert!(has_gin(&hand_from_str("TH TC TS TD").unwrap()));
    }

    #[test]
    fn test_wraparound() {
        assert!(!has_gin(&hand_from_str("KD AD 2D").unwrap()));
    }

    #[test]
    fn test_double_dip() {
        assert!(!has_gin(&hand_from_str("6H 6C 6D 7D 8D").unwrap()));
    }

    #[test]
    fn test_full_hand() {
        assert!(has_gin(
            &hand_from_str("7H 8H 9H TH 6H 6D 6S KH KD KC").unwrap()
        ));
    }
}
