use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    htype: HandType,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.htype.cmp(&other.htype)  // todo we must consider cards too, if we have matching htypes
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.htype == other.htype
    }
}

impl Eq for Hand {}

impl Hand {
    fn new(cards: &str) -> Hand {
        let cards_vec = cards.chars().collect::<Vec<char>>();
        Hand {
            cards: cards_vec.clone(),
            htype: Hand::compute_rank(cards_vec),
        }
    }

    fn compute_rank(cards: Vec<char>) -> HandType {
        // Count each card in the hand
        let mut counts = HashMap::<char, i32>::new();
        for c in cards.clone() {
            *counts.entry(c).or_insert(0) += 1;
        }

        // Exhaustively check possible combinations
        let mut count_values = counts.values().collect::<Vec<&i32>>();
        count_values.sort();
        count_values.reverse();

        for card_count in count_values {
            match card_count {
                5 => return HandType::FiveOfAKind,
                4 => return HandType::FourOfAKind,
                3 => {
                    // Full house & 3 of a kind check
                    if counts.len() == 2 {
                        return HandType::FullHouse;
                    } else {
                        return HandType::ThreeOfAKind;
                    }
                }
                1 => {
                    if counts.len() == 3 {
                        return HandType::TwoPair;
                    } else if counts.len() == 4 {
                        return HandType::OnePair;
                    } else {
                        return HandType::HighCard;
                    }
                }
                _ => continue,
            };
        }
        panic!("Failed to find the rank for {:?}!", cards);
    }
}

fn main() {
    // Camel Poker

    // Part 1
    println!("{:?}", Hand::new("AAAAA"));

    // let examples =
}

#[cfg(test)]
mod tests {
    use crate::HandType;
    use crate::Hand;

    #[test]
    fn test_hand_type_ordering() {
        let mut shuffled_hand_types = vec![
            HandType::TwoPair,
            HandType::FourOfAKind,
            HandType::FiveOfAKind,
            HandType::HighCard,
            HandType::ThreeOfAKind,
            HandType::OnePair,
            HandType::FullHouse,
        ];

        let expected_order = vec![
            HandType::FiveOfAKind,
            HandType::FourOfAKind,
            HandType::FullHouse,
            HandType::ThreeOfAKind,
            HandType::TwoPair,
            HandType::OnePair,
            HandType::HighCard,
        ];

        assert_ne!(expected_order, shuffled_hand_types);

        shuffled_hand_types.sort();

        assert_eq!(expected_order, shuffled_hand_types);
    }

    #[test]
    fn test_hand_type_ordering_and_eq() {
        let mut shuffled_hands = vec![
            Hand::new("AAAAB"),
            Hand::new("AABCD"),
            Hand::new("AAAAA"),
            Hand::new("ABCDE"),
            Hand::new("AAABC"),
            Hand::new("AABBC"),
            Hand::new("AAABB"),
        ];

        let expected_order = vec![
            Hand::new("AAAAA"),
            Hand::new("AAAAB"),
            Hand::new("AAABB"),
            Hand::new("AAABC"),
            Hand::new("AABBC"),
            Hand::new("AABCD"),
            Hand::new("ABCDE"),
        ];

        assert_ne!(expected_order, shuffled_hands);

        shuffled_hands.sort();

        assert_eq!(expected_order, shuffled_hands);
    }
}
