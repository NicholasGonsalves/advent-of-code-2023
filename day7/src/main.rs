use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap};

fn custom_card_order(c: &char) -> u8 {
    match c {
        'A' => 1,
        'K' => 2,
        'Q' => 3,
        'J' => 4,
        'T' => 5,
        '9' => 6,
        '8' => 7,
        '7' => 8,
        '6' => 9,
        '5' => 10,
        '4' => 11,
        '3' => 12,
        '2' => 13,
        _ => panic!("{} isn't a card in camel poker!", c),
    }
}

fn custom_card_order_joker(c: &char) -> u8 {
    match c {
        'A' => 1,
        'K' => 2,
        'Q' => 3,
        'T' => 4,
        '9' => 5,
        '8' => 6,
        '7' => 7,
        '6' => 8,
        '5' => 9,
        '4' => 10,
        '3' => 11,
        '2' => 12,
        'J' => 13,
        _ => panic!("{} isn't a card in camel poker!", c),
    }
}

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
    bid: u32,
    joker_style: bool,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let primary_cmp = self.htype.cmp(&other.htype);

        // If htypes properties are equal, compare based on actual cards
        if primary_cmp == std::cmp::Ordering::Equal {
            for i in 0..self.cards.len() {
                if self.joker_style {
                    match custom_card_order_joker(self.cards.get(i).unwrap())
                        .cmp(&custom_card_order_joker(&other.cards.get(i).unwrap()))
                    {
                        Ordering::Equal => continue,
                        result => return result,
                    }
                } else {
                    match custom_card_order(self.cards.get(i).unwrap())
                        .cmp(&custom_card_order(&other.cards.get(i).unwrap()))
                    {
                        Ordering::Equal => continue,
                        result => return result,
                    }
                }
            }

            // If all specified positions are equal, consider the structs equal
            Ordering::Equal
        } else {
            primary_cmp
        }
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
    fn new(cards: &str, bid: u32) -> Hand {
        let cards_vec = cards.chars().collect::<Vec<char>>();
        Hand {
            cards: cards_vec.clone(),
            htype: Hand::compute_rank(cards_vec),
            bid: bid,
            joker_style: false,
        }
    }

    fn new_joker_style(cards: &str, bid: u32) -> Hand {
        let cards_vec = cards.chars().collect::<Vec<char>>();
        Hand {
            cards: cards_vec.clone(),
            htype: Hand::compute_rank_joker_style(cards_vec),
            bid: bid,
            joker_style: true,
        }
    }

    fn default(cards: &str) -> Hand {
        Hand::new(cards, 0)
    }

    fn from_line(line: &str) -> Hand {
        let (cards, bid_str) = line.split_once(" ").unwrap();
        Hand::new(cards, bid_str.parse::<u32>().unwrap())
    }

    fn from_line_joker_style(line: &str) -> Hand {
        let (cards, bid_str) = line.split_once(" ").unwrap();
        Hand::new_joker_style(cards, bid_str.parse::<u32>().unwrap())
    }

    fn compute_rank(cards: Vec<char>) -> HandType {
        // Count each card in the hand
        let mut counts = HashMap::<char, i32>::new();
        for c in cards.clone() {
            *counts.entry(c).or_insert(0) += 1;
        }

        // Exhaustively check possible combinations
        let mut count_values = counts.values().collect::<Vec<&i32>>();
        // Iterate in reverse sorted order, to force largest card counts to appear first
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

    // (part 2) Now cards can be jokers
    fn compute_rank_joker_style(cards: Vec<char>) -> HandType {
        // Count each card in the hand
        let mut counts = HashMap::<char, i32>::new();
        for c in cards.clone() {
            *counts.entry(c).or_insert(0) += 1;
        }

        // Specifically check the number of jokers (they act as wildcards!)
        let joker_count = &counts.get(&'J').unwrap_or(&0).clone();

        if joker_count == &5 {
            return HandType::FiveOfAKind; // 5 Jokers!
        }

        counts.remove(&'J');

        // Exhaustively check possible combinations
        let mut count_values = counts.values().collect::<Vec<&i32>>();
        // Iterate in reverse sorted order, to force largest card counts to appear first
        count_values.sort();
        count_values.reverse();

        for card_count in count_values {
            match card_count + joker_count {
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
                2 => {
                    if joker_count == &0 {
                        if counts.len() == 3 {
                            return HandType::TwoPair;
                        } else if counts.len() == 4 {
                            return HandType::OnePair;
                        }
                    } else {
                        return HandType::OnePair;
                    }
                }
                1 => {
                    return HandType::HighCard;
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
    let winnings_part_1: u32 = include_str!("day7.txt")
        .lines()
        .map(Hand::from_line)
        .sorted()
        .rev()
        .enumerate()
        .map(|(rank, hand)| hand.bid * (rank + 1) as u32)
        .sum();

    println!("{:?}", winnings_part_1);

    // Part 2
    let winnings_part_2: u32 = include_str!("day7.txt")
        .lines()
        .map(Hand::from_line_joker_style)
        .sorted()
        .rev()
        .enumerate()
        .map(|(rank, hand)| hand.bid * (rank + 1) as u32)
        .sum();

    println!("{:?}", winnings_part_2);
}

#[cfg(test)]
mod tests {
    use crate::Hand;
    use crate::HandType;

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
            Hand::default("AAAAB"),
            Hand::default("AABCD"),
            Hand::default("22222"),
            Hand::default("AAAAA"),
            Hand::default("ABCDE"),
            Hand::default("AAABC"),
            Hand::default("AABBC"),
            Hand::default("AAABB"),
        ];

        let expected_order = vec![
            Hand::default("AAAAA"), // Note, AAAAA and 22222 have same type,
            Hand::default("22222"), // so ordering is based on our custom_card_order func!
            Hand::default("AAAAB"),
            Hand::default("AAABB"),
            Hand::default("AAABC"),
            Hand::default("AABBC"),
            Hand::default("AABCD"),
            Hand::default("ABCDE"),
        ];

        assert_ne!(expected_order, shuffled_hands);

        shuffled_hands.sort();

        assert_eq!(expected_order, shuffled_hands);
    }
}
